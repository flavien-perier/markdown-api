use std::fs::read_to_string;
use std::path::Path as FilePath;

use crate::functions::is_valid_markdown_filename::is_valid_markdown_filename;
use axum::{
    extract::Path,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};

pub async fn get_markdown_file(Path(filename): Path<String>) -> impl IntoResponse {
    let empty_response: Response<String> = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("File not found".into())
        .unwrap();

    if !is_valid_markdown_filename(&filename) {
        return empty_response;
    }

    let file_path = std::env::current_dir()
        .unwrap_or_else(|_| FilePath::new(".").to_path_buf())
        .join("documents")
        .join(filename);

    let markdown_content = match read_to_string(&file_path) {
        Ok(content) => content,
        Err(_) => return empty_response,
    };

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/markdown; charset=utf-8")
        .body(markdown_content.into())
        .unwrap()
}
