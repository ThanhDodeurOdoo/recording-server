use std::sync::{Arc, Mutex};
use std::thread;
use tokio;
use env_logger;

mod config;
mod services;
mod models;
mod misc;

use config::{ initialize_required_globals };
use models::bus::Bus;
use services::Service;
use crate::services::http::HttpService;


fn boot_service<S, F>(name: String, bus: Arc<Mutex<Bus>>, service_factory: F)
where
    S: Service + Send + Sync + 'static,
    F: FnOnce(String, Arc<Mutex<Bus>>) -> S + Send + 'static,
{
    let bus_for_service = Arc::clone(&bus);

    tokio::spawn(async move {
        // Create the service using the factory function
        let service = service_factory(name, bus_for_service);
        // Start the service (registers, listens, and does any service-specific work)
        Arc::new(service).start().await;
    });
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    initialize_required_globals();
    let bus = Arc::new(Mutex::new(Bus::new()));
    boot_service("http_service".to_string(), Arc::clone(&bus), |name, bus| {
        HttpService::new(name, bus)
    });
    thread::park();
    Ok(())
}
