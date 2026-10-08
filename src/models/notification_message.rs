use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationMessage {
    #[serde(flatten)]
    pub notification_base: models::NotificationBase,
    /// A JSON-encoded string.
    #[serde(rename = "details")]
    pub details: String,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl NotificationMessage {
    pub fn new(
        notification_base: models::NotificationBase,
        details: String,
        r#type: Type,
    ) -> NotificationMessage {
        NotificationMessage {
            notification_base,
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
