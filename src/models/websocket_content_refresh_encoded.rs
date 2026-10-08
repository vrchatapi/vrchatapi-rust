use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketContentRefreshEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketContentRefresh,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketContentRefreshEncoded {
    pub fn new(
        content: models::WebsocketContentRefresh,
        r#type: Type,
    ) -> WebsocketContentRefreshEncoded {
        WebsocketContentRefreshEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "content-refresh")]
    ContentRefresh,
}

impl Default for Type {
    fn default() -> Type {
        Self::ContentRefresh
    }
}
