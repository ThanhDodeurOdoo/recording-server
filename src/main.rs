mod services;
mod models;
mod misc;

pub use services::http;
use tokio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    http::start();
    Ok(())
}
