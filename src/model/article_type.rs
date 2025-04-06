use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum ArticleType {
    Article,
    Blog,
    Documentation,
    Wiki,
}
