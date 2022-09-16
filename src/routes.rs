use axum::routing::get;
use axum::Router;

async fn root() -> &'static str {
    "Chirpity"
}

// async fn registration_form() {}

pub fn register_routes(router: Router) -> Router {
    router.route("/", get(root))
    // .route("/register", get(registration_form()))
}
