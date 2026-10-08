use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketResponseNotification {
    #[serde(rename = "notificationId")]
    pub notification_id: String,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "receiverId")]
    pub receiver_id: String,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "responseId")]
    pub response_id: String,
}

impl WebsocketResponseNotification {
    pub fn new(
        notification_id: String,
        receiver_id: String,
        response_id: String,
    ) -> WebsocketResponseNotification {
        WebsocketResponseNotification {
            notification_id,
            receiver_id,
            response_id,
        }
    }
}
