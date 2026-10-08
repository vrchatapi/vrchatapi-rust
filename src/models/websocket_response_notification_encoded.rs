use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketResponseNotificationEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketResponseNotification,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketResponseNotificationEncoded {
    pub fn new(
        content: models::WebsocketResponseNotification,
        r#type: Type,
    ) -> WebsocketResponseNotificationEncoded {
        WebsocketResponseNotificationEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "response-notification")]
    ResponseNotification,
}

impl Default for Type {
    fn default() -> Type {
        Self::ResponseNotification
    }
}
