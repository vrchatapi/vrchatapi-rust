use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationBoop {
    #[serde(flatten)]
    pub notification_base: models::NotificationBase,
    /// A JSON-encoded string.
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "details")]
    pub details: models::NotificationDetailBoop,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl NotificationBoop {
    pub fn new(
        notification_base: models::NotificationBase,
        details: models::NotificationDetailBoop,
        r#type: Type,
    ) -> NotificationBoop {
        NotificationBoop {
            notification_base,
            details,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "boop")]
    Boop,
}

impl Default for Type {
    fn default() -> Type {
        Self::Boop
    }
}
