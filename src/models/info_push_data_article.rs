use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataArticle {
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<models::InfoPushDataArticleContent>,
}

impl InfoPushDataArticle {
    pub fn new() -> InfoPushDataArticle {
        InfoPushDataArticle { content: None }
    }
}
