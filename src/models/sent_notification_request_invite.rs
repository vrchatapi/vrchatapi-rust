use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentNotificationRequestInvite {
    #[serde(flatten)]
    pub sent_notification_base: models::SentNotificationBase,
    #[serde(rename = "details")]
    pub details: models::NotificationDetailRequestInvite,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl SentNotificationRequestInvite {
    pub fn new(
        sent_notification_base: models::SentNotificationBase,
        details: models::NotificationDetailRequestInvite,
        r#type: Type,
    ) -> SentNotificationRequestInvite {
        SentNotificationRequestInvite {
            sent_notification_base,
            details,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "requestInvite")]
    RequestInvite,
}

impl Default for Type {
    fn default() -> Type {
        Self::RequestInvite
    }
}
