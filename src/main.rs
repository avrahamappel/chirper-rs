use axum::routing::get;
use axum::{Router, Server};

async fn root() -> &'static str {
    "Chirpity"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let url = "0.0.0.0:8000";

    Server::bind(&url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
