#![warn(clippy::pedantic)]

mod config;
mod misc;
mod models;
mod services;

use config::initialize_required_globals;
use services::http;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    initialize_required_globals();
    http::start().await;
    std::thread::park();
    Ok(())
}
