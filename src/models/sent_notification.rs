use crate::models;
use serde::{Deserialize, Serialize};

/// SentNotification :
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentNotification {
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "details")]
    pub details: models::SentNotificationDetails,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "message")]
    pub message: String,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "receiverUserId")]
    pub receiver_user_id: String,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "senderUserId")]
    pub sender_user_id: String,
    /// The name of the user who sent the notification.
    #[serde(rename = "senderUsername", skip_serializing_if = "Option::is_none")]
    pub sender_username: Option<String>,
    #[serde(rename = "type")]
    pub r#type: models::NotificationType,
}

impl SentNotification {
    pub fn new(
        created_at: chrono::DateTime<chrono::FixedOffset>,
        details: models::SentNotificationDetails,
        id: String,
        message: String,
        receiver_user_id: String,
        sender_user_id: String,
        r#type: models::NotificationType,
    ) -> SentNotification {
        SentNotification {
            created_at,
            details,
            id,
            message,
            receiver_user_id,
            sender_user_id,
            sender_username: None,
            r#type,
        }
    }
}
