use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketSeeNotification {
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketSeeNotification {
    pub fn new(content: String, r#type: Type) -> WebsocketSeeNotification {
        WebsocketSeeNotification { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "see-notification")]
    SeeNotification,
}

impl Default for Type {
    fn default() -> Type {
        Self::SeeNotification
    }
}
