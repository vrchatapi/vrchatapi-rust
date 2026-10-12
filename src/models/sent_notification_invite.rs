use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentNotificationInvite {
    #[serde(flatten)]
    pub sent_notification_base: models::SentNotificationBase,
    #[serde(rename = "details")]
    pub details: models::NotificationDetailInvite,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl SentNotificationInvite {
    pub fn new(
        sent_notification_base: models::SentNotificationBase,
        details: models::NotificationDetailInvite,
        r#type: Type,
    ) -> SentNotificationInvite {
        SentNotificationInvite {
            sent_notification_base,
            details,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "invite")]
    Invite,
}

impl Default for Type {
    fn default() -> Type {
        Self::Invite
    }
}
