pub mod config;
pub mod routes;

use std::net::SocketAddr;

use axum::Router;
use config::Config;
use eyre::Result;
use routes::add_routes;

pub async fn run(config: Config) -> Result<()> {
    let app = add_routes(Router::new());
    let addr = SocketAddr::from((config.address, config.port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
