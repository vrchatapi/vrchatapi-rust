use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketUserUpdateEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketUserUpdate,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketUserUpdateEncoded {
    pub fn new(content: models::WebsocketUserUpdate, r#type: Type) -> WebsocketUserUpdateEncoded {
        WebsocketUserUpdateEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "user-update")]
    UserUpdate,
}

impl Default for Type {
    fn default() -> Type {
        Self::UserUpdate
    }
}
