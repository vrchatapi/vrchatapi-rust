use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketNotificationEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketNotification,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketNotificationEncoded {
    pub fn new(
        content: models::WebsocketNotification,
        r#type: Type,
    ) -> WebsocketNotificationEncoded {
        WebsocketNotificationEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "notification")]
    Notification,
}

impl Default for Type {
    fn default() -> Type {
        Self::Notification
    }
}
