use crate::model::markdown_header::MarkdownHeader;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub pages: usize,
    pub files: Vec<MarkdownHeader>,
}
