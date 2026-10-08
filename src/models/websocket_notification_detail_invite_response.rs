use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketNotificationDetailInviteResponse {
    #[serde(flatten)]
    pub notification_base: models::NotificationBase,
    #[serde(rename = "details")]
    pub details: models::NotificationDetailInviteResponse,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketNotificationDetailInviteResponse {
    pub fn new(
        notification_base: models::NotificationBase,
        details: models::NotificationDetailInviteResponse,
        r#type: Type,
    ) -> WebsocketNotificationDetailInviteResponse {
        WebsocketNotificationDetailInviteResponse {
            notification_base,
            details,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "invite-response")]
    InviteResponse,
}

impl Default for Type {
    fn default() -> Type {
        Self::InviteResponse
    }
}
