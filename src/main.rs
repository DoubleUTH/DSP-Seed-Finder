#![cfg(not(target_arch = "wasm32"))]

mod data;
mod rules;
mod tests;
mod transform_rules;
mod worldgen;

use data::game_desc::GameDesc;
use futures_util::{SinkExt, StreamExt};
use rayon::iter::{ParallelExtend, ParallelIterator};
use rayon::slice::ParallelSlice;
use rayon::{ThreadPool, ThreadPoolBuilder};
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use transform_rules::Rules;
use worldgen::galaxy_gen::{create_galaxy, find_stars};

use crate::data::galaxy::Galaxy;
use crate::data::rule::Rule;
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
    SearchStar {
        seed: i32,
        game: GameDesc,
        rule: Rules,
    },
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum OutgoingMessage<'a> {
    Generate { galaxy: Galaxy<'a> },
    Setup { success: bool },
    SearchStar { indexes: Vec<usize> },
}

struct SetupData {
    pub pool: ThreadPool,
    pub game: GameDesc,
    pub rule: Arc<Box<dyn Rule + Send + Sync>>,
}

async fn handle_message(msg: IncomingMessage, current_setup: &mut Option<SetupData>) -> String {
    match msg {
        IncomingMessage::Generate { seed, game } => tokio::task::spawn_blocking(move || {
            let habitable_count = Cell::new(0_i32);
            let galaxy = create_galaxy(seed, &game, &habitable_count);
            serde_json::to_string(&OutgoingMessage::Generate { galaxy }).unwrap()
        })
        .await
        .unwrap(),
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
            serde_json::to_string(&OutgoingMessage::Setup { success: true }).unwrap()
        }
        IncomingMessage::SearchStar { seed, game, rule } => {
            tokio::task::spawn_blocking(move || {
                let transformed_rule = transform_rules(rule);
                let star_indexes = find_stars(seed, &game, &transformed_rule);
                serde_json::to_string(&OutgoingMessage::SearchStar {
                    indexes: star_indexes,
                })
                .unwrap()
            })
            .await
            .unwrap()
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
                let resp = handle_message(msg, &mut current_setup).await;
                let _ = tx.send(Message::Text(resp.into())).await;
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
                let game = setup.game;
                let rule = Arc::clone(&setup.rule);
                setup.pool.spawn(move || {
                    let mut result = bytes[..4].to_vec();

                    let iter = bytes[4..]
                        .par_chunks_exact(4)
                        .filter(move |chunk| {
                            let array: [u8; 4] = (*chunk).try_into().unwrap();
                            let seed = i32::from_ne_bytes(array);
                            let star_indexes = find_stars(seed, &game, &rule);
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
