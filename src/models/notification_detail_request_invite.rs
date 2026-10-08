use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationDetailRequestInvite {
    /// TODO: Does this still exist?
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// Used when using InviteMessage Slot.
    #[serde(rename = "requestMessage", skip_serializing_if = "Option::is_none")]
    pub request_message: Option<String>,
}

impl NotificationDetailRequestInvite {
    pub fn new() -> NotificationDetailRequestInvite {
        NotificationDetailRequestInvite {
            platform: None,
            request_message: None,
        }
    }
}
