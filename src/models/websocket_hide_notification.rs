use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketHideNotification {
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketHideNotification {
    pub fn new(content: String, r#type: Type) -> WebsocketHideNotification {
        WebsocketHideNotification { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "hide-notification")]
    HideNotification,
}

impl Default for Type {
    fn default() -> Type {
        Self::HideNotification
    }
}
