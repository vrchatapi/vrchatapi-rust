use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketGroupLeftEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketGroupLeft,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketGroupLeftEncoded {
    pub fn new(content: models::WebsocketGroupLeft, r#type: Type) -> WebsocketGroupLeftEncoded {
        WebsocketGroupLeftEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "group-left")]
    GroupLeft,
}

impl Default for Type {
    fn default() -> Type {
        Self::GroupLeft
    }
}
