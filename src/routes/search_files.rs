use std::path::Path as FilePath;
use tokio::fs::read_to_string;

use crate::functions::is_valid_markdown_filename::is_valid_markdown_filename;
use crate::model::article_type::ArticleType;
use crate::model::markdown_header::MarkdownHeader;
use crate::model::search_result::SearchResult;
use axum::Json;
use axum::extract::Query;
use regex::Regex;
use serde::Deserialize;
use tracing::log;
use crate::functions::configuration::get_domain;

#[derive(Deserialize)]
pub struct QueryParams {
    p: Option<i32>,
    n: Option<i32>,
    categ: Option<String>,
    #[serde(rename = "type")]
    article_type: Option<ArticleType>,
    q: Option<String>,
}

pub fn extract_header(content: &str, filename: String, url: String) -> Option<MarkdownHeader> {
    let markdown_header_regex: Regex = Regex::new(r"(?s)^---\n(.*?)\n---").unwrap();

    if let Some(captures) = markdown_header_regex.captures(content) {
        let metadata = &captures[1];
        match serde_yaml::from_str::<MarkdownHeader>(metadata) {
            Ok(mut header) => {
                header.filename = Some(filename);
                header.url = Some(url);
                Some(header)
            }
            Err(err) => {
                log::error!("In document {} failed to parse YAML metadata : {}", filename, err);
                None
            }
        }
    } else {
        None
    }
}

pub async fn search_files(Query(params): Query<QueryParams>) -> Json<SearchResult> {
    let dir_path = std::env::current_dir()
        .unwrap_or_else(|_| FilePath::new(".").to_path_buf())
        .join("documents");

    if !dir_path.exists() || !dir_path.is_dir() {
        log::error!(
            "The directory '{}' does not exist or is not a directory.",
            dir_path.display()
        );
        return Json(SearchResult {
            pages: 0,
            files: vec![],
        });
    }

    let mut read_dir = tokio::fs::read_dir(&dir_path).await.unwrap();

    let n = params.n.unwrap_or(10) as usize;
    let p = params.p.unwrap_or(1) as usize;
    let filter = params.q.clone().unwrap_or_default();
    let categ = params.categ.clone().unwrap_or_default();

    let filter_regex = Regex::new(&filter).unwrap_or_else(|_| Regex::new(".*").unwrap());

    let mut all_files = vec![];

    while let Some(entry) = read_dir.next_entry().await.unwrap() {
        let file_path = entry.path();

        let filename = file_path.file_name().unwrap().to_string_lossy().to_string();
        let url = format!("{}/{}", get_domain(), filename);

        if file_path.is_file() && is_valid_markdown_filename(entry.file_name().to_str().unwrap()) {
            if let Ok(content) = read_to_string(&file_path).await {
                if let Some(header) = extract_header(&content, filename, url) {
                    let matches_filter = filter_regex.is_match(&header.title)
                        || filter_regex.is_match(&header.description);
                    let matches_article_type = params
                        .article_type
                        .as_ref()
                        .map_or(true, |article_type| header.article_type == *article_type);
                    let matches_category =
                        categ.is_empty() || header.categories.iter().any(|cat| cat == &categ);

                    if matches_filter && matches_article_type && matches_category {
                        all_files.push(header);
                    }
                }
            }
        }
    }

    let start = (p - 1) * n;
    let paginated_files: Vec<_> = all_files.clone().into_iter().skip(start).take(n).collect();

    let mut files = vec![];
    files.extend(paginated_files);
    files.sort_by(|a, b| b.date.cmp(&a.date));

    let result = SearchResult {
        pages: (all_files.len() + n - 1) / n,
        files,
    };

    Json(result)
}
