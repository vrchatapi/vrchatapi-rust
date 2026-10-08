use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationInviteResponse {
    #[serde(flatten)]
    pub notification_base: models::NotificationBase,
    /// A JSON-encoded string.
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "details")]
    pub details: models::NotificationDetailInviteResponse,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl NotificationInviteResponse {
    pub fn new(
        notification_base: models::NotificationBase,
        details: models::NotificationDetailInviteResponse,
        r#type: Type,
    ) -> NotificationInviteResponse {
        NotificationInviteResponse {
            notification_base,
            details,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "inviteResponse")]
    InviteResponse,
}

impl Default for Type {
    fn default() -> Type {
        Self::InviteResponse
    }
}
