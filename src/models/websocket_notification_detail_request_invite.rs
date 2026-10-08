use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketNotificationDetailRequestInvite {
    #[serde(flatten)]
    pub notification_base: models::NotificationBase,
    #[serde(rename = "details")]
    pub details: models::NotificationDetailRequestInvite,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketNotificationDetailRequestInvite {
    pub fn new(
        notification_base: models::NotificationBase,
        details: models::NotificationDetailRequestInvite,
        r#type: Type,
    ) -> WebsocketNotificationDetailRequestInvite {
        WebsocketNotificationDetailRequestInvite {
            notification_base,
            details,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "request-invite")]
    RequestInvite,
}

impl Default for Type {
    fn default() -> Type {
        Self::RequestInvite
    }
}
