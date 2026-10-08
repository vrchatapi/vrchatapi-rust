use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketNotificationV2UpdateEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketNotificationV2Update,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketNotificationV2UpdateEncoded {
    pub fn new(
        content: models::WebsocketNotificationV2Update,
        r#type: Type,
    ) -> WebsocketNotificationV2UpdateEncoded {
        WebsocketNotificationV2UpdateEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "notification-v2-update")]
    NotificationV2Update,
}

impl Default for Type {
    fn default() -> Type {
        Self::NotificationV2Update
    }
}
