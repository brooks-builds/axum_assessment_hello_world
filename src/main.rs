use std::net::SocketAddr;

use axum::{routing::get, Router};

/// The entire server is defined in the main function here. To pass the assessment create files and/or directories to put the hello world server code in. The main function should only call a function that runs the server. It's okay for main to create and set up a configuration object and pass that in.
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> &'static str {
    "hello world"
}
