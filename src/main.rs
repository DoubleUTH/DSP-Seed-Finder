#![cfg(not(target_arch = "wasm32"))]

mod data;
mod rules;
mod transform_rules;
mod worldgen;

use data::game_desc::GameDesc;
use futures_util::lock::Mutex;
use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Handle;
use tokio::sync::mpsc;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use transform_rules::Rules;
use worldgen::galaxy_gen::{create_galaxy, find_stars};

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), std::io::Error> {
    println!("Starting...");
    let listener = TcpListener::bind("127.0.0.1:62879").await?;
    println!("Started.");
    println!("You may now turn on native mode to search.");
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum IncomingMessage {
    Generate {
        game: GameDesc,
    },
    Find {
        game: GameDesc,
        rule: Rules,
        range: (i32, i32),
        concurrency: i32,
        autosave: u64,
    },
    Stop,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum OutgoingMessage {
    Result { seed: i32, indexes: Vec<usize> },
    Progress { start: i32, end: i32 },
    Done { start: i32, end: i32 },
}

struct FindState {
    pub progress_start: i32,
    pub progress_end: i32,
    pub pending_seeds: HashSet<i32>,
    pub autosave: u64,
    pub last_notify: SystemTime,
}

const SEED_BATCH_SIZE: i32 = 100;

impl FindState {
    pub fn add_all(&mut self, start: i32) -> Option<(i32, i32)> {
        if self.progress_end == start {
            self.progress_end = start + SEED_BATCH_SIZE;
            let mut e = self.progress_end;
            while self.pending_seeds.remove(&e) {
                e += SEED_BATCH_SIZE;
            }
            self.progress_end = e;
            let now = SystemTime::now();
            if now.duration_since(self.last_notify).unwrap().as_secs() >= self.autosave {
                self.last_notify = now;
                let start = self.progress_start;
                self.progress_start = e;
                Some((start, e))
            } else {
                None
            }
        } else {
            self.pending_seeds.insert(start);
            None
        }
    }
}

enum WorkerMessage {
    Message { text: String },
    Done,
}

async fn accept_connection(stream: TcpStream) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during websocket handshake");
    let (write, read) = ws_stream.split();

    let boxed_write = Arc::new(Mutex::new(write));
    let stopped = Arc::new(AtomicBool::new(false));

    let _ = read
        .try_for_each(|msg| {
            if !msg.is_empty() {
                let msg: IncomingMessage = serde_json::from_str(&msg.to_string()).unwrap();
                match msg {
                    IncomingMessage::Stop => {
                        println!("Stopping");
                        stopped.store(true, Ordering::SeqCst);
                    }
                    IncomingMessage::Generate { game } => {
                        let w = boxed_write.clone();
                        tokio::task::spawn_blocking(move || {
                            let galaxy = create_galaxy(&game);
                            let output = serde_json::to_string(&galaxy).unwrap();
                            let runtime = Handle::current();
                            runtime.block_on(async move {
                                w.lock()
                                    .await
                                    .send(Message::Text(output.into()))
                                    .await
                                    .unwrap();
                            })
                        });
                    }
                    IncomingMessage::Find {
                        game,
                        rule,
                        range: (start, end),
                        concurrency,
                        autosave,
                    } => {
                        println!("Receive search request.");
                        println!("Concurrency: {}.", concurrency);
                        let threads = concurrency.min(end - start);
                        let current_seed = Arc::new(AtomicI32::new(start));
                        let state = Arc::new(std::sync::Mutex::new(FindState {
                            progress_end: start,
                            progress_start: start,
                            pending_seeds: HashSet::new(),
                            autosave,
                            last_notify: SystemTime::now(),
                        }));
                        stopped.store(false, Ordering::SeqCst);
                        let (tx, mut rx) = mpsc::channel::<WorkerMessage>(1000);

                        for _ in 0..threads {
                            let tx = tx.clone();
                            let transformed = transform_rules::transform_rules(rule.clone());
                            let mut g = game.clone();
                            let s = state.clone();
                            let cs = current_seed.clone();
                            let stop = stopped.clone();
                            tokio::task::spawn_blocking(move || {
                                loop {
                                    let batch_start = cs
                                        .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| {
                                            Some(x + SEED_BATCH_SIZE)
                                        })
                                        .unwrap();
                                    if batch_start >= end {
                                        break;
                                    }
                                    let batch_end = (batch_start + SEED_BATCH_SIZE).min(end);

                                    for seed in batch_start..batch_end {
                                        g.seed = seed;
                                        let star_indexes = find_stars(&g, &transformed);
                                        if !star_indexes.is_empty() {
                                            let output =
                                                serde_json::to_string(&OutgoingMessage::Result {
                                                    seed,
                                                    indexes: star_indexes,
                                                })
                                                .unwrap();
                                            let _ = tx.blocking_send(WorkerMessage::Message {
                                                text: output,
                                            });
                                        }
                                    }

                                    let notify_progress = {
                                        let mut x = s.lock().unwrap();
                                        x.add_all(batch_start)
                                    };
                                    if let Some((start, end)) = notify_progress {
                                        println!("Processing: {}.", end);
                                        let output =
                                            serde_json::to_string(&OutgoingMessage::Progress {
                                                start,
                                                end,
                                            })
                                            .unwrap();
                                        let _ = tx
                                            .blocking_send(WorkerMessage::Message { text: output });
                                    }
                                    if stop.load(Ordering::SeqCst) {
                                        break;
                                    }
                                }
                                let _ = tx.blocking_send(WorkerMessage::Done);
                            });
                        }
                        drop(tx);

                        let w = boxed_write.clone();
                        let state_for_completion = state.clone();
                        tokio::spawn(async move {
                            let mut finished_threads = 0;

                            while let Some(msg) = rx.recv().await {
                                match msg {
                                    WorkerMessage::Message { text } => {
                                        let _ =
                                            w.lock().await.send(Message::Text(text.into())).await;
                                    }
                                    WorkerMessage::Done => {
                                        finished_threads += 1;
                                        if finished_threads == threads {
                                            let (progress_start, progress_end) = {
                                                let x = state_for_completion.lock().unwrap();
                                                (x.progress_start, x.progress_end)
                                            };
                                            println!("Completed: {}.", progress_end);
                                            let output =
                                                serde_json::to_string(&OutgoingMessage::Done {
                                                    start: progress_start,
                                                    end: progress_end,
                                                })
                                                .unwrap();
                                            let _ = w
                                                .lock()
                                                .await
                                                .send(Message::Text(output.into()))
                                                .await;
                                            break;
                                        }
                                    }
                                }
                            }
                        });
                    }
                }
            }
            future::ok(())
        })
        .await;
}
