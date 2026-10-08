use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationVoteToKick {
    #[serde(flatten)]
    pub notification_base: models::NotificationBase,
    /// A JSON-encoded string.
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "details")]
    pub details: models::NotificationDetailVoteToKick,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl NotificationVoteToKick {
    pub fn new(
        notification_base: models::NotificationBase,
        details: models::NotificationDetailVoteToKick,
        r#type: Type,
    ) -> NotificationVoteToKick {
        NotificationVoteToKick {
            notification_base,
            details,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "votetokick")]
    Votetokick,
}

impl Default for Type {
    fn default() -> Type {
        Self::Votetokick
    }
}
