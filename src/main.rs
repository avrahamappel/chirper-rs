use std::sync::Arc;

use axum::{Extension, Router, Server};

mod routes;
mod state;

use state::State;

#[tokio::main]
async fn main() {
    let state_layer = Extension(Arc::new(State::new()));
    let app = routes::register_routes(Router::new().layer(state_layer));

    let url = "0.0.0.0:8000";

    Server::bind(&url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
