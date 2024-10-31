use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, Result};
use actix_web::rt::System;
use serde::{Serialize, Deserialize};
use log::{info, warn, error};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use actix_web::http::header::ContentType;
use uuid::Uuid;

use crate::models;
use crate::misc;
use crate::config;

use misc::enums::{ WsCloseCode };
use models::channel::{ Channel, SharedChannels, ChannelStats };
use config::{ AUTH_KEY, HTTP_INTERFACE, PORT };

const API_VERSION: u8 = 1;

#[derive(Serialize)]
struct NoopResponse {
    result: String,
}

async fn noop_handler() -> Result<HttpResponse> {
    let response = NoopResponse {
        result: "ok".to_string(),
    };
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(response))
}

async fn stats_handler(channels: web::Data<SharedChannels>) -> Result<HttpResponse> {
    let channels_lock = channels.lock().unwrap();
    let channel_stats: Vec<ChannelStats> = channels_lock
        .values()
        .map(|channel| channel.get_stats())
        .collect();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(channel_stats))
}

#[derive(Deserialize)]
struct ChannelParams {
    webRTC: Option<String>,
}

#[derive(Serialize)]
struct ChannelResponse {
    uuid: String,
    url: String,
}

async fn channel_handler(
    req: HttpRequest,
    query: web::Query<ChannelParams>,
    channels: web::Data<SharedChannels>,
) -> Result<HttpResponse> {
    let remote_addr = req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("")
        .to_string();
    let host = req.connection_info().host().to_string();
    let protocol = req.connection_info().scheme().to_string();

    let auth_header = req.headers().get("Authorization");
    if let Some(auth_value) = auth_header {
        if let Ok(auth_str) = auth_value.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return match auth_verify(token) {
                    Ok(claims) => {
                        if claims.iss.is_empty() {
                            warn!(
                                "{}: missing issuer claim when creating channel",
                                remote_addr
                            );
                            return Ok(HttpResponse::Forbidden().finish());
                        }
                        let channel = Channel::create(
                            &remote_addr,
                            &claims.iss,
                            claims.key,
                            query.webRTC.as_deref() != Some("false"),
                            channels.get_ref().clone(),
                        );
                        let response = ChannelResponse {
                            uuid: channel.uuid.clone(),
                            url: format!("{}://{}", protocol, host),
                        };
                        Ok(HttpResponse::Ok()
                            .content_type(ContentType::json())
                            .json(response))
                    }
                    Err(err) => {
                        warn!("[{}] failed to create channel: {}", remote_addr, err);
                        Ok(HttpResponse::Unauthorized().finish())
                    }
                }
            }
        }
    }
    warn!(
        "[{}] failed to create channel: missing or invalid authorization header",
        remote_addr
    );
    Ok(HttpResponse::Unauthorized().finish())
}

#[derive(Deserialize)]
struct DisconnectClaims {
    sessionIdsByChannel: HashMap<String, Vec<u32>>,
}

async fn disconnect_handler(
    req: HttpRequest,
    body: web::Bytes,
    channels: web::Data<SharedChannels>,
) -> Result<HttpResponse> {
    let remote_addr = req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("")
        .to_string();

    let jwt = String::from_utf8(body.to_vec()).unwrap();
    match auth_verify(&jwt) {
        Ok(claims) => {
            if let Some(session_ids_by_channel) = claims.sessionIdsByChannel {
                for (channel_uuid, session_ids) in session_ids_by_channel {
                    let mut channels_lock = channels.lock().unwrap();
                    if let Some(channel) = channels_lock.get_mut(&channel_uuid) {
                        if channel.remote_address != remote_addr {
                            warn!(
                                "[{}] tried to disconnect sessions from channel {} but is not the owner",
                                remote_addr, channel_uuid
                            );
                            continue;
                        }
                        for session_id in session_ids {
                            if let Some(session) = channel.sessions.get(&session_id) {
                                session.close(
                                    WsCloseCode::Kicked,
                                    &format!("/disconnect by {}", remote_addr),
                                );
                            }
                        }
                    }
                }
                Ok(HttpResponse::Ok().finish())
            } else {
                error!(
                    "[{}] failed to disconnect session: sessionIdsByChannel missing in claims",
                    remote_addr
                );
                Ok(HttpResponse::UnprocessableEntity().finish())
            }
        }
        Err(err) => {
            error!("[{}] failed to disconnect session: {}", remote_addr, err);
            Ok(HttpResponse::UnprocessableEntity().finish())
        }
    }
}

#[derive(Debug, Deserialize)]
struct Claims {
    iss: String,
    key: Option<String>,
    #[serde(default)]
    sessionIdsByChannel: Option<HashMap<String, Vec<u32>>>,
}

fn auth_verify(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(AUTH_KEY.as_ref()),
        &validation,
    )?;
    Ok(token_data.claims)
}

pub async fn http() -> std::io::Result<()> {
    info!("starting...");
    let channels: SharedChannels = Arc::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(channels.clone()))
            .route(
                &format!("/v{}/noop", API_VERSION),
                web::get().to(noop_handler),
            )
            .route(
                &format!("/v{}/stats", API_VERSION),
                web::get().to(stats_handler),
            )
            .route(
                &format!("/v{}/channel", API_VERSION),
                web::get().to(channel_handler),
            )
            .route(
                &format!("/v{}/disconnect", API_VERSION),
                web::post().to(disconnect_handler),
            )
    })
        .bind((&**HTTP_INTERFACE, *PORT))?
        .run()
        .await
}

pub fn start() -> std::thread::JoinHandle<()> {
    std::thread::spawn(|| {
        let sys = System::new();
        sys.block_on(async {
            if let Err(e) = http().await {
                eprintln!("Server error: {}", e);
            }
        });
    })
}