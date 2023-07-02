use std::net::SocketAddr;

use axum::{routing::get, Router};
use eyre::Result;
use hello_world::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new();

    match hello_world::run(config).await {
        Ok(_) => println!("Server shut down"),
        Err(error) => eprintln!("Something went wrong: {}", error.to_string()),
    }

    Ok(())
}
