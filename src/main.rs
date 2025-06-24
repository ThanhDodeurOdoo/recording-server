#![warn(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

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
    http::start().await
}
