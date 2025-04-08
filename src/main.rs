use axum::http::Method;
use axum::routing::{Router, get};
use tower_http::cors::{Any, CorsLayer};
use tracing::log;

mod model;
mod routes;
mod functions;
mod parser;

use crate::routes::get_markdown_file::get_markdown_file;
use crate::routes::search_files::search_files;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET]);

    let app = Router::new()
        .route("/", get(search_files))
        .route("/{filename}", get(get_markdown_file))
        .layer(cors);

    let addr = "0.0.0.0:8080".to_string();

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    log::info!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
