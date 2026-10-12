use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentNotificationVoteToKick {
    #[serde(flatten)]
    pub sent_notification_base: models::SentNotificationBase,
    #[serde(rename = "details")]
    pub details: models::NotificationDetailVoteToKick,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl SentNotificationVoteToKick {
    pub fn new(
        sent_notification_base: models::SentNotificationBase,
        details: models::NotificationDetailVoteToKick,
        r#type: Type,
    ) -> SentNotificationVoteToKick {
        SentNotificationVoteToKick {
            sent_notification_base,
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
