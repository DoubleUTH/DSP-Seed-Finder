mod data;
mod rules;
mod worldgen;

use std::sync::Arc;

use data::game_desc::GameDesc;
use futures_util::lock::Mutex;
use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use worldgen::galaxy_gen::create_galaxy;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:9000").await?;
    while let Ok((stream, _)) = listener.accept().await {
        spawn(accept_connection(stream)).await?;
    }
    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during websocket handshake");
    let (write, read) = ws_stream.split();

    let boxed_write = Arc::new(Mutex::new(write));

    read.try_for_each(|msg| {
        if !msg.is_empty() {
            let str = msg.to_string();
            let w = boxed_write.clone();
            tokio::task::spawn(async move {
                let game_desc: GameDesc = serde_json::from_str(&str).unwrap();
                let galaxy = create_galaxy(&game_desc);
                let output = serde_json::to_string(&galaxy).unwrap();
                w.lock().await.send(Message::Text(output)).await.unwrap();
            });
        }
        future::ok(())
    })
    .await
    .unwrap();
}
