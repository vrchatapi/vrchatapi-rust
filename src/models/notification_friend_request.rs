use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationFriendRequest {
    #[serde(flatten)]
    pub notification_base: models::NotificationBase,
    /// A JSON-encoded string.
    #[serde(rename = "details")]
    pub details: String,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl NotificationFriendRequest {
    pub fn new(
        notification_base: models::NotificationBase,
        details: String,
        r#type: Type,
    ) -> NotificationFriendRequest {
        NotificationFriendRequest {
            notification_base,
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
