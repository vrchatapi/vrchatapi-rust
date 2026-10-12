use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentNotificationFriendRequest {
    #[serde(flatten)]
    pub sent_notification_base: models::SentNotificationBase,
    #[serde(rename = "details")]
    pub details: serde_json::Value,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl SentNotificationFriendRequest {
    pub fn new(
        sent_notification_base: models::SentNotificationBase,
        details: serde_json::Value,
        r#type: Type,
    ) -> SentNotificationFriendRequest {
        SentNotificationFriendRequest {
            sent_notification_base,
            details,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "friendRequest")]
    FriendRequest,
}

impl Default for Type {
    fn default() -> Type {
        Self::FriendRequest
    }
}
