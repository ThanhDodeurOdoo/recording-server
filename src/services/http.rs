use std::collections::HashMap;
use axum::{
    routing::{get},
    extract::ws::{Message, WebSocketUpgrade, WebSocket},
    response::{Html, IntoResponse},
    http::StatusCode,
    Router, Json,
};
use serde::Serialize;
use futures_util::StreamExt;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use log::{info};
use tokio::spawn;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::config::{ HTTP_INTERFACE, PORT };
use crate::misc::flatbuffer_types;
use crate::models::remote::{ Remote, REMOTES };


const API_VERSION: u8 = 1;

#[derive(Serialize)]
struct NoopResponse {
    result: String,
}

async fn noop_handler() -> impl IntoResponse {
    let response = NoopResponse {
        result: "ok".to_string(),
    };
    Json(response)
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let remote_address = "remote_address_example".to_string(); // Replace with actual remote address
    let remote = Arc::new(Remote::new(remote_address.clone()));
    {
        let mut remotes = REMOTES.lock().unwrap();
        remotes.insert(remote_address.clone(), remote.clone());
    }
    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            Message::Ping(ping) => {
                if socket.send(Message::Pong(ping)).await.is_err() {
                    return;
                }
            }
            Message::Text(text) => println!("ws message/text: {text}"),
            Message::Binary(bin) => {
                // Parse the flatbuffer message
                let data = flatbuffers::root::<flatbuffer_types::recording::Message>(&bin).unwrap();
                match data.type_() {
                    "recording" => {
                        let recording = data.content_as_recording_payload().unwrap();
                        let channel_uuid = recording.channel_uuid();
                        let capabilities = recording.capabilities();
                        let transport_port = recording.transport_port();
                        let transport_host = recording.transport_host();
                        let mut remotes = REMOTES.lock().unwrap();
                        if let Some(remote) = remotes.get_mut(&remote_address) {
                            remote.start_recording(channel_uuid);
                        }
                    }
                    "command" => {
                        let command = data.content_as_command_payload().unwrap();
                        let channel_uuid = command.channel_uuid();
                        let name = command.name();
                    }
                    _ => println!("Unknown type"),
                }

            }
            _ => break,
        }
    }
    // Remove the remote when the socket is closed
    let mut remotes = REMOTES.lock().unwrap();
    remotes.remove(&remote_address);
}

pub async fn start() {
    let app = Router::new()
        .route(
            "/noop",
            get(noop_handler),
        )
        .route("/ws", get(ws_handler));
    let listener = tokio::net::TcpListener::bind((&**HTTP_INTERFACE, *PORT)).await.unwrap();
    let addr = listener.local_addr().unwrap();
    info!("Server running at {addr}");
    axum::serve(listener, app).await.unwrap();
}
