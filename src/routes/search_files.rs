use std::path::Path as FilePath;
use tokio::fs::read_to_string;

use crate::functions::is_valid_markdown_filename::is_valid_markdown_filename;
use crate::model::article_type::ArticleType;
use crate::model::markdown_header::MarkdownHeader;
use crate::model::search_result::SearchResult;
use axum::Json;
use axum::extract::Query;
use regex::{Regex, RegexBuilder};
use serde::Deserialize;
use std::sync::LazyLock;
use tracing::log;
use crate::functions::configuration::get_domain;

const DEFAULT_PAGE_SIZE: usize = 10;
const MAX_PAGE_SIZE: usize = 100;
const MAX_QUERY_LENGTH: usize = 256;
const MAX_REGEX_SIZE: usize = 1 << 20;

static MARKDOWN_HEADER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)^---\r?\n(.*?)\r?\n---").unwrap());

fn build_filter_regex(query: Option<&str>) -> Option<Regex> {
    let query = query.filter(|q| !q.is_empty())?;
    let query: String = query.chars().take(MAX_QUERY_LENGTH).collect();
    // Invalid or too complex patterns disable the filter (match everything).
    RegexBuilder::new(&query)
        .size_limit(MAX_REGEX_SIZE)
        .dfa_size_limit(MAX_REGEX_SIZE)
        .build()
        .ok()
}

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
    if let Some(captures) = MARKDOWN_HEADER_REGEX.captures(content) {
        let metadata = &captures[1];
        match serde_saphyr::from_str::<MarkdownHeader>(metadata) {
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

    let mut read_dir = match tokio::fs::read_dir(&dir_path).await {
        Ok(read_dir) => read_dir,
        Err(err) => {
            log::error!("Failed to read directory '{}': {}", dir_path.display(), err);
            return Json(SearchResult {
                pages: 0,
                files: vec![],
            });
        }
    };

    let n = params
        .n
        .and_then(|n| usize::try_from(n).ok())
        .filter(|n| *n > 0)
        .unwrap_or(DEFAULT_PAGE_SIZE)
        .min(MAX_PAGE_SIZE);
    let p = params
        .p
        .and_then(|p| usize::try_from(p).ok())
        .filter(|p| *p > 0)
        .unwrap_or(1);
    let categ = params.categ.clone().unwrap_or_default();

    let filter_regex = build_filter_regex(params.q.as_deref());
    let domain = get_domain();

    let mut all_files = vec![];

    while let Ok(Some(entry)) = read_dir.next_entry().await {
        let file_path = entry.path();

        let Some(filename) = entry.file_name().to_str().map(str::to_string) else {
            continue;
        };

        if file_path.is_file() && is_valid_markdown_filename(&filename) {
            if let Ok(content) = read_to_string(&file_path).await {
                let url = format!("{}/{}", domain, filename);
                if let Some(header) = extract_header(&content, filename, url) {
                    let matches_filter = filter_regex.as_ref().map_or(true, |re| {
                        re.is_match(&header.title) || re.is_match(&header.description)
                    });
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

    all_files.sort_by(|a, b| b.date.cmp(&a.date));

    let pages = all_files.len().div_ceil(n);
    let start = (p - 1).saturating_mul(n);
    let files: Vec<_> = all_files.into_iter().skip(start).take(n).collect();

    let result = SearchResult { pages, files };

    Json(result)
}
