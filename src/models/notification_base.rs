use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationBase {
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "message")]
    pub message: String,
    /// Not included in notification objects received from the REST API
    #[serde(rename = "receiverUserId", skip_serializing_if = "Option::is_none")]
    pub receiver_user_id: Option<String>,
    /// Not included in notification objects received from the Websocket API
    #[serde(rename = "seen", skip_serializing_if = "Option::is_none")]
    pub seen: Option<bool>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "senderUserId")]
    pub sender_user_id: String,
    /// The name of the user who sent the notification.
    #[serde(
        rename = "senderUsername",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sender_username: Option<Option<String>>,
}

impl NotificationBase {
    pub fn new(
        created_at: chrono::DateTime<chrono::FixedOffset>,
        id: String,
        message: String,
        sender_user_id: String,
    ) -> NotificationBase {
        NotificationBase {
            created_at,
            id,
            message,
            receiver_user_id: None,
            seen: None,
            sender_user_id,
            sender_username: None,
        }
    }
}
