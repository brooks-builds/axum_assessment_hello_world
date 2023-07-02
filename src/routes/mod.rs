mod hello_world;

use axum::{routing::get, Router};

pub fn add_routes(router: Router) -> Router {
    router.route("/", get(hello_world::hello_world))
}
