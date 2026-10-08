use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketNotificationV2Update {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "updates")]
    pub updates: serde_json::Value,
    #[serde(rename = "version")]
    pub version: i32,
}

impl WebsocketNotificationV2Update {
    pub fn new(
        id: String,
        updates: serde_json::Value,
        version: i32,
    ) -> WebsocketNotificationV2Update {
        WebsocketNotificationV2Update {
            id,
            updates,
            version,
        }
    }
}
