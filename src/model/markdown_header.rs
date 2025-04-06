use crate::model::article_type::ArticleType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownHeader {
    pub title: String,
    #[serde(rename = "type")]
    pub article_type: ArticleType,
    pub categories: Vec<String>,
    pub description: String,
    pub author: String,
    pub date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
