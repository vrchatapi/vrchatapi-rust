use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketNotificationV2DeleteEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketNotificationV2Delete,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketNotificationV2DeleteEncoded {
    pub fn new(
        content: models::WebsocketNotificationV2Delete,
        r#type: Type,
    ) -> WebsocketNotificationV2DeleteEncoded {
        WebsocketNotificationV2DeleteEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "notification-v2-delete")]
    NotificationV2Delete,
}

impl Default for Type {
    fn default() -> Type {
        Self::NotificationV2Delete
    }
}
