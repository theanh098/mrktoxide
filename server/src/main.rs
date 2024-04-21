mod error;
mod extractors;
mod handlers;

use axum::{routing::get, Router};
use extractors::AppState;
use handlers::{get_collections, get_listed_nfts, get_user_nfts};

#[tokio::main]

async fn main() {
    dotenv::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("db_url must be set");
    let redis_url = "redis://127.0.0.1/";

    let app = Router::new()
        .route("/collections", get(get_collections))
        .route(
            "/collections/:collection_address/nfts",
            get(get_listed_nfts),
        )
        .route("/users/:address/nfts", get(get_user_nfts))
        .with_state(AppState::init(&db_url, redis_url).await);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
