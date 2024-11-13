// local modules
use crate::logs::init_logger;

// modules
use tokio::net::{ TcpListener, TcpStream };
use tokio_tungstenite::{ accept_async, tungstenite::protocol::Message };
use futures::{ StreamExt, SinkExt };
use std::net::SocketAddr;
use log::{ info, error };
use std::env;

async fn handle_connection(stream: TcpStream) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("Error during the websocket handshake: {}", e);
            return;
        }
    };

    // Split the WebSocket stream into a sender and receiver
    let (mut sender, mut receiver) = ws_stream.split();

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let reversed = text.chars().rev().collect::<String>();
                if let Err(e) = sender.send(Message::Text(reversed)).await {
                    error!("Error sending message: {}", e);
                }
            }
            Ok(Message::Close(_)) => {
                break;
            }
            Ok(_) => (),
            Err(e) => {
                error!("Error processing message: {}", e);
                break;
            }
        }
    }
}

pub async fn bootstrap() {
    init_logger();

    let address = env
        ::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let address: SocketAddr = address.parse().expect("Invalid address!");

    let listener = TcpListener::bind(&address).await.expect("Failed to bind address.");

    info!("Listening on: {}", address);

    while let Ok((stream, _)) = listener.accept().await {
        // Spawns a new task for each connection.
        tokio::spawn(handle_connection(stream));
    }
}
