use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentNotificationInviteResponse {
    #[serde(flatten)]
    pub sent_notification_base: models::SentNotificationBase,
    #[serde(rename = "details")]
    pub details: models::NotificationDetailInviteResponse,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl SentNotificationInviteResponse {
    pub fn new(
        sent_notification_base: models::SentNotificationBase,
        details: models::NotificationDetailInviteResponse,
        r#type: Type,
    ) -> SentNotificationInviteResponse {
        SentNotificationInviteResponse {
            sent_notification_base,
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
