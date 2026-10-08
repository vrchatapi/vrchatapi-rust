use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketNotificationV2Encoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::NotificationV2,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketNotificationV2Encoded {
    pub fn new(content: models::NotificationV2, r#type: Type) -> WebsocketNotificationV2Encoded {
        WebsocketNotificationV2Encoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "notification-v2")]
    NotificationV2,
}

impl Default for Type {
    fn default() -> Type {
        Self::NotificationV2
    }
}
