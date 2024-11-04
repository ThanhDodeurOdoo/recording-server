use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, Result};
use serde::{Deserialize, Serialize};
use log::{info, error};
use std::sync::{Arc, Mutex};
use actix_web::http::header::ContentType;

use crate::models;
use crate::misc;
use crate::config;

use config::{ HTTP_INTERFACE, PORT };
use misc::auth::verify;
use models::bus::Bus;
use crate::services::Service;

const API_VERSION: u8 = 1;

#[derive(Serialize)]
struct NoopResponse {
    result: String,
}

#[derive(Deserialize)]
struct Claims {
    id: String,
}

pub struct HttpService {
    pub bus: Arc<Mutex<Bus>>,
    pub name: String,
}

impl Clone for HttpService {
    fn clone(&self) -> Self {
        HttpService {
            bus: Arc::clone(&self.bus),
            name: self.name.clone(),
        }
    }
}

impl Service for HttpService {
    fn name(&self) -> &str {
        &self.name
    }

    fn bus(&self) -> Arc<Mutex<Bus>> {
        Arc::clone(&self.bus)
    }

    fn handle_message(&self, message: String) {
        println!("[{}] Handling message: {}", self.name(), message);
        // Custom handling logic can go here
    }

    async fn start(self: Arc<Self>) {
        info!("Starting HTTP Service...");
        let data = web::Data::new(self.clone());
        HttpServer::new(move || {
            App::new()
                .app_data(data.clone())
                .route(
                    &format!("/v{}/noop", API_VERSION),
                    web::get().to(Self::noop_handler),
                )
                .route(
                    &format!("/v{}/stats", API_VERSION),
                    web::get().to(Self::stats_handler),
                )
                .route(
                    &format!("/v{}/record", API_VERSION),
                    web::post().to(Self::record_handler),
                )
        })
            .bind((&**HTTP_INTERFACE, *PORT))
            .expect("Failed to bind to address")
            .run()
            .await
            .expect("HTTP server failed");
    }
}

impl HttpService {
    pub fn new(name: String, bus: Arc<Mutex<Bus>>) -> Self {
        HttpService { name, bus }
    }
    pub async fn noop_handler(data: web::Data<HttpService>) -> Result<HttpResponse> {
        let response = NoopResponse {
            result: String::from("ok"),
        };
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(response))
    }

    async fn stats_handler(data: web::Data<HttpService>) -> Result<HttpResponse> {
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .json("TODO"))
    }

    async fn record_handler(
        data: web::Data<HttpService>,
        req: HttpRequest,
        body: web::Bytes,
    ) -> Result<HttpResponse> {
        let remote_addr = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("")
            .to_string();
        let jwt = String::from_utf8(body.to_vec()).unwrap();
        match verify::<Claims>(&jwt) {
            Ok(..) => { // claims
                    Ok(HttpResponse::Ok().finish())
            }
            Err(err) => {
                error!("[{}] failed to disconnect session: {}", remote_addr, err);
                Ok(HttpResponse::UnprocessableEntity().finish())
            }
        }
    }
}
