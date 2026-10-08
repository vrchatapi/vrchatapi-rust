use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketNotificationV2Delete {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "version")]
    pub version: i32,
}

impl WebsocketNotificationV2Delete {
    pub fn new(id: String, version: i32) -> WebsocketNotificationV2Delete {
        WebsocketNotificationV2Delete { id, version }
    }
}
