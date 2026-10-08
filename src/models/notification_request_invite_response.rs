use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationRequestInviteResponse {
    #[serde(flatten)]
    pub notification_base: models::NotificationBase,
    /// A JSON-encoded string.
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "details")]
    pub details: models::NotificationDetailRequestInviteResponse,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl NotificationRequestInviteResponse {
    pub fn new(
        notification_base: models::NotificationBase,
        details: models::NotificationDetailRequestInviteResponse,
        r#type: Type,
    ) -> NotificationRequestInviteResponse {
        NotificationRequestInviteResponse {
            notification_base,
            details,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "requestInviteResponse")]
    RequestInviteResponse,
}

impl Default for Type {
    fn default() -> Type {
        Self::RequestInviteResponse
    }
}
