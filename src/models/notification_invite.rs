use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationInvite {
    #[serde(flatten)]
    pub notification_base: models::NotificationBase,
    /// A JSON-encoded string.
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "details")]
    pub details: models::NotificationDetailInvite,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl NotificationInvite {
    pub fn new(
        notification_base: models::NotificationBase,
        details: models::NotificationDetailInvite,
        r#type: Type,
    ) -> NotificationInvite {
        NotificationInvite {
            notification_base,
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
