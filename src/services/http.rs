use actix_web::{
    get,
    middleware::Logger,
    web::{self, Json},
    App, HttpRequest, HttpServer, Responder,
};
use log::info;
use serde::Serialize;

use crate::config::{HTTP_INTERFACE, PORT};
use crate::models::remote::Remote;

#[derive(Serialize)]
struct NoopResponse {
    result: &'static str,
}

#[get("/noop")]
async fn noop_handler() -> Json<NoopResponse> {
    Json(NoopResponse { result: "ok" })
}

#[get("/ws")]
async fn ws_handler(
    req: HttpRequest,
    body: web::Payload,
) -> actix_web::Result<impl Responder> {
    let remote_addr = req
        .connection_info()
        .peer_addr()
        .unwrap_or("unknown")
        .to_string();
    
    let (response, session, msg_stream) = actix_ws::handle(&req, body)?;
    actix_web::rt::spawn(handle_socket(session, msg_stream, remote_addr));
    
    Ok(response)
}

async fn handle_socket(
    session: actix_ws::Session,
    msg_stream: actix_ws::MessageStream,
    remote_address: String,
) {
    let mut remote = Remote::new(remote_address.clone(), session, msg_stream);
    info!("created remote: {}", remote_address);
    remote.listen().await;
}

#[allow(clippy::unwrap_used)] // we can safely unwrap here, as we know the values are set and if they are not, we want to panic
pub async fn start() -> std::io::Result<()> {
    info!("Server running at {}:{}", &*HTTP_INTERFACE, *PORT);
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(noop_handler)
            .service(ws_handler)
    })
    .bind((&**HTTP_INTERFACE, *PORT))?
    .run()
    .await
}
