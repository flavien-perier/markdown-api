use axum::routing::{Router, get};
use tracing::log;

mod functions;
mod model;
mod routes;

use crate::routes::get_markdown_file::get_markdown_file;
use crate::routes::search_files::search_files;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let app = Router::new()
        .route("/", get(search_files))
        .route("/:filename", get(get_markdown_file));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    log::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
