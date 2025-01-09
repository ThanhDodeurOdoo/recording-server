use axum::extract::ConnectInfo;
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use log::info;
use serde::Serialize;
use std::net::SocketAddr;

use crate::config::{HTTP_INTERFACE, PORT};
use crate::models::remote::Remote;

#[derive(Serialize)]
struct NoopResponse {
    result: &'static str,
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(socket: WebSocket, remote_address: SocketAddr) {
    Remote::new(remote_address.to_string(), socket).listen().await;
}

#[allow(clippy::unwrap_used)] // we can safely unwrap here, as we know the values are set and if they are not, we want to panic
pub async fn start() {
    #[rustfmt::skip]
    let app = Router::new()
        .route("/noop", get(|| async { Json(NoopResponse { result: "ok" }) }))
        .route("/ws", get(ws_handler));

    let listener = tokio::net::TcpListener::bind((&**HTTP_INTERFACE, *PORT)).await.unwrap();
    let addr = listener.local_addr().unwrap();
    info!("Server running at {addr}");
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("http server error: {e}");
    }
}
