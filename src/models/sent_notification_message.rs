use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentNotificationMessage {
    #[serde(flatten)]
    pub sent_notification_base: models::SentNotificationBase,
    #[serde(rename = "details")]
    pub details: serde_json::Value,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl SentNotificationMessage {
    pub fn new(
        sent_notification_base: models::SentNotificationBase,
        details: serde_json::Value,
        r#type: Type,
    ) -> SentNotificationMessage {
        SentNotificationMessage {
            sent_notification_base,
            details,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "message")]
    Message,
}

impl Default for Type {
    fn default() -> Type {
        Self::Message
    }
}
