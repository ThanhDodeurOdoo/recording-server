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
use log::{info};
use tokio::spawn;
use crate::config::{ HTTP_INTERFACE, PORT };

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
    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            Message::Ping(ping) => {
                if socket.send(Message::Pong(ping)).await.is_err() {
                    return;
                }
            }
            Message::Text(text) => println!("ws message/text: {text}"),
            Message::Binary(bin) => println!("ws message/binary: {:?}", bin),
            _ => break,
        }
    }
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
