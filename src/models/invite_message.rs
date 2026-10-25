use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InviteMessage {
    #[serde(rename = "canBeUpdated")]
    pub can_be_updated: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "messageType")]
    pub message_type: models::InviteMessageType,
    /// Changes to 60 when updated, although probably server-side configurable.
    #[serde(rename = "remainingCooldownMinutes")]
    pub remaining_cooldown_minutes: i32,
    #[serde(rename = "slot")]
    pub slot: i32,
    /// A date and time in the form of an RFC 3339 `date-time`, except that the UTC offset can be written without its colon, as in `+0000`.
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl InviteMessage {
    pub fn new(
        can_be_updated: bool,
        id: String,
        message: String,
        message_type: models::InviteMessageType,
        remaining_cooldown_minutes: i32,
        slot: i32,
        updated_at: chrono::DateTime<chrono::FixedOffset>,
    ) -> InviteMessage {
        InviteMessage {
            can_be_updated,
            id,
            message,
            message_type,
            remaining_cooldown_minutes,
            slot,
            updated_at,
        }
    }
}
