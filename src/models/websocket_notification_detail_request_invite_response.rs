use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketNotificationDetailRequestInviteResponse {
    #[serde(flatten)]
    pub notification_base: models::NotificationBase,
    #[serde(rename = "details")]
    pub details: models::NotificationDetailRequestInviteResponse,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketNotificationDetailRequestInviteResponse {
    pub fn new(
        notification_base: models::NotificationBase,
        details: models::NotificationDetailRequestInviteResponse,
        r#type: Type,
    ) -> WebsocketNotificationDetailRequestInviteResponse {
        WebsocketNotificationDetailRequestInviteResponse {
            notification_base,
            details,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "request-invite-response")]
    RequestInviteResponse,
}

impl Default for Type {
    fn default() -> Type {
        Self::RequestInviteResponse
    }
}
