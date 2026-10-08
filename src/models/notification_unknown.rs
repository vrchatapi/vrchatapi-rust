use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationUnknown {
    #[serde(flatten)]
    pub notification_base: models::NotificationBase,
    /// A JSON-encoded string.
    #[serde(rename = "details")]
    pub details: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl NotificationUnknown {
    pub fn new(
        notification_base: models::NotificationBase,
        details: String,
        r#type: String,
    ) -> NotificationUnknown {
        NotificationUnknown {
            notification_base,
            details,
            r#type,
        }
    }
}
