use tokio;
use env_logger;

mod config;
mod services;
mod models;
mod misc;

use services::http;
use config::{ initialize_required_globals };

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    initialize_required_globals();
    http::start().await;
    std::thread::park();
    Ok(())
}
