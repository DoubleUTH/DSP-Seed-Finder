#![cfg(not(target_arch = "wasm32"))]

mod data;
mod rules;
mod worldgen;
mod transform_rules;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering, AtomicI32};
use data::rule::Rule;
use futures_util::future::join_all;
use serde::{Deserialize, Serialize};
use data::game_desc::GameDesc;
use futures_util::lock::Mutex;
use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use transform_rules::Rules;
use worldgen::galaxy_gen::{create_galaxy, find_stars};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:9000").await?;
    while let Ok((stream, _)) = listener.accept().await {
        spawn(accept_connection(stream)).await?;
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum IncomingMessage {
    Generate { game: GameDesc },
    Find { game: GameDesc, rule: Rules, range: (i32, i32), concurrency: i32 },
    Stop,
}

async fn accept_connection(stream: TcpStream) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during websocket handshake");
    let (write, read) = ws_stream.split();

    let boxed_write = Arc::new(Mutex::new(write));
    let stopped = Arc::new(AtomicBool::new(false));

    read.try_for_each(|msg| {
        if !msg.is_empty() {
            let msg: IncomingMessage = serde_json::from_str(&msg.to_string()).unwrap();
            match msg {
                IncomingMessage::Stop => {
                    stopped.store(true, Ordering::SeqCst);
                },
                IncomingMessage::Generate { game } => {
                    let w: Arc<Mutex<futures_util::stream::SplitSink<tokio_tungstenite::WebSocketStream<TcpStream>, Message>>> = boxed_write.clone();
                    tokio::task::spawn(async move {
                        let galaxy = create_galaxy(&game);
                        let output = serde_json::to_string(&galaxy).unwrap();
                        w.lock().await.send(Message::Text(output)).await.unwrap();
                    });
                }
                IncomingMessage::Find { game, rule, range: (start, end), concurrency } => {
                    let current_seed = Arc::new(AtomicI32::new(start));
                    let threads = concurrency.min(end - start + 1);
                    stopped.store(false, Ordering::SeqCst);
                    for _ in 0..threads {
                        let w = boxed_write.clone();
                        let mut transformed: Box<dyn Rule + Send> = transform_rules::transform_rules(rule.clone());
                        let mut g = game.clone();
                        let s = current_seed.clone();
                        let stop = stopped.clone();
                        tokio::task::spawn_blocking(move || {
                            loop {
                                let seed = s.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)).unwrap();
                                if seed > end {
                                    break;
                                }
                                g.seed = seed;
                                let galaxy = find_stars(&g, &mut transformed);
                                let output = serde_json::to_string(&galaxy).unwrap();
                                // w.lock().await.send(Message::Text(output)).await.unwrap();
                                if stop.load(Ordering::SeqCst) {
                                    break;
                                }
                            }
                        });
                    }
                }
            }
        }
        future::ok(())
    })
    .await
    .unwrap();
}
