#![cfg(not(target_arch = "wasm32"))]

mod data;
mod database;
mod rules;
mod tests;
mod transform_rules;
mod worldgen;

use data::game_desc::GameDesc;
use futures_util::{SinkExt, StreamExt};
use rayon::iter::{ParallelBridge, ParallelExtend, ParallelIterator};
use rayon::slice::ParallelSlice;
use rayon::{ThreadPool, ThreadPoolBuilder};
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use transform_rules::Rules;
use worldgen::galaxy_gen::{create_galaxy, find_stars};

use crate::data::galaxy::Galaxy;
use crate::data::rule::Rule;
use crate::database::{
    create_database, create_insert_seed_stmt, get_seed_params, insert_seed, set_info, SeedParams,
};
use crate::transform_rules::transform_rules;

#[tokio::main(flavor = "multi_thread")]
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
        seed: i32,
        game: GameDesc,
    },
    Setup {
        concurrency: usize,
        game: GameDesc,
        rule: Rules,
    },
    Database {
        name: String,
        concurrency: usize,
        range: (i32, i32),
        game: GameDesc,
    },
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum OutgoingMessage<'a> {
    Generate { galaxy: Galaxy<'a> },
    Setup { success: bool },
    Database { success: bool, progress: i32 },
}

struct SetupData {
    pub pool: ThreadPool,
    pub game: GameDesc,
    pub rule: Arc<Box<dyn Rule + Send + Sync>>,
}

const DATABASE_CHUNK_SIZE: usize = 100;

fn generate_database(name: String, range: &(i32, i32), game: &GameDesc) -> rusqlite::Result<bool> {
    let mut conn = create_database(name)?;
    if !set_info(&mut conn, range, game)? {
        return Ok(false);
    }
    let (start, end) = *range;
    let use_actual_veins = game.use_actual_veins;
    let (tx, mut rx) = mpsc::channel::<Vec<SeedParams>>(1000);
    rayon::scope(|s| {
        s.spawn(move |_| {
            let mut buffer = Vec::with_capacity(DATABASE_CHUNK_SIZE);
            loop {
                buffer.clear();
                let count = rx.blocking_recv_many(&mut buffer, DATABASE_CHUNK_SIZE);
                if count == 0 {
                    break;
                }
                let mut trans = conn.transaction().unwrap();
                {
                    let mut stmt = create_insert_seed_stmt(&mut trans).unwrap();
                    for params in buffer.iter().take(count) {
                        insert_seed(&mut stmt, params);
                    }
                }
                trans.commit().unwrap();
            }
        });
        let _ = (start..end).par_bridge().for_each(move |seed| {
            let tx = tx.clone();
            let habitable_count = Cell::new(0_i32);
            let galaxy = create_galaxy(seed, game, &habitable_count);
            // println!("{}", seed);
            let _ = tx.blocking_send(get_seed_params(&galaxy, use_actual_veins));
        });
    });
    Ok(true)
}

async fn handle_message(
    msg: IncomingMessage,
    current_setup: &mut Option<SetupData>,
    tx: &Sender<Message>,
) {
    match msg {
        IncomingMessage::Generate { seed, game } => {
            let resp = tokio::task::spawn_blocking(move || {
                let habitable_count = Cell::new(0_i32);
                let galaxy = create_galaxy(seed, &game, &habitable_count);
                serde_json::to_string(&OutgoingMessage::Generate { galaxy }).unwrap()
            })
            .await
            .unwrap();
            let _ = tx.send(Message::Text(resp.into())).await;
        }
        IncomingMessage::Setup {
            concurrency,
            game,
            rule,
        } => {
            *current_setup = Some(SetupData {
                pool: ThreadPoolBuilder::new()
                    .num_threads(concurrency)
                    .build()
                    .unwrap(),
                game,
                rule: Arc::new(transform_rules(rule)),
            });
            let resp = serde_json::to_string(&OutgoingMessage::Setup { success: true }).unwrap();
            let _ = tx.send(Message::Text(resp.into())).await;
        }
        IncomingMessage::Database {
            name,
            concurrency,
            range,
            game,
        } => {
            let pool = ThreadPoolBuilder::new()
                .num_threads(concurrency)
                .build()
                .unwrap();
            let tx = tx.clone();
            pool.spawn(move || {
                let res = generate_database(name, &range, &game);
                match res {
                    Ok(_) => {
                        let resp = serde_json::to_string(&&OutgoingMessage::Database {
                            success: true,
                            progress: range.1,
                        })
                        .unwrap();
                        let _ = tx.blocking_send(Message::Text(resp.into()));
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                        let resp = serde_json::to_string(&&OutgoingMessage::Database {
                            success: false,
                            progress: 0,
                        })
                        .unwrap();
                        let _ = tx.blocking_send(Message::Text(resp.into()));
                    }
                };
            });
        }
    }
}

async fn accept_connection(stream: TcpStream) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during websocket handshake");
    let (mut write, mut read) = ws_stream.split();

    let mut current_setup = None;
    let (tx, mut rx) = mpsc::channel::<Message>(1000);

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let _ = write.send(msg).await;
        }
    });

    while let Some(result) = read.next().await {
        let msg = result?;
        match msg {
            Message::Text(text) => {
                let msg: IncomingMessage = serde_json::from_str(&text).unwrap();
                handle_message(msg, &mut current_setup, &tx).await;
            }
            Message::Binary(bytes) => {
                // println!("Receive search request for {} seeds", size);
                if bytes.len() < 8 {
                    eprintln!("Warning: Received malformed binary packet.");
                    continue;
                } else if current_setup.is_none() {
                    eprintln!("Warning: Received search request before setup.");
                    continue;
                }
                let tx = tx.clone();
                let setup = current_setup.as_ref().unwrap();
                let game = setup.game.clone();
                let rule = Arc::clone(&setup.rule);
                setup.pool.spawn(move || {
                    let mut result = bytes[..4].to_vec();

                    let iter = bytes[4..]
                        .par_chunks_exact(4)
                        .filter(move |chunk| {
                            let array: [u8; 4] = (*chunk).try_into().unwrap();
                            let seed = i32::from_ne_bytes(array);
                            let habitable_count = Cell::new(0_i32);
                            let star_indexes = find_stars(seed, &game, &habitable_count, &rule);
                            !star_indexes.is_empty()
                        })
                        .flatten()
                        .copied();

                    result.par_extend(iter);

                    let _ = tx.blocking_send(Message::Binary(result.into()));
                });
            }
            _ => {}
        }
    }

    Ok(())
}
