use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentNotificationBoop {
    #[serde(flatten)]
    pub sent_notification_base: models::SentNotificationBase,
    #[serde(rename = "details")]
    pub details: models::NotificationDetailBoop,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl SentNotificationBoop {
    pub fn new(
        sent_notification_base: models::SentNotificationBase,
        details: models::NotificationDetailBoop,
        r#type: Type,
    ) -> SentNotificationBoop {
        SentNotificationBoop {
            sent_notification_base,
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
