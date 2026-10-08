use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketNotificationDetailVoteToKick {
    #[serde(flatten)]
    pub notification_base: models::NotificationBase,
    #[serde(rename = "details")]
    pub details: models::NotificationDetailVoteToKick,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketNotificationDetailVoteToKick {
    pub fn new(
        notification_base: models::NotificationBase,
        details: models::NotificationDetailVoteToKick,
        r#type: Type,
    ) -> WebsocketNotificationDetailVoteToKick {
        WebsocketNotificationDetailVoteToKick {
            notification_base,
            details,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "vote-to-kick")]
    VoteToKick,
}

impl Default for Type {
    fn default() -> Type {
        Self::VoteToKick
    }
}
