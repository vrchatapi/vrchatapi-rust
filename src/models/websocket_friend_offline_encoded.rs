use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketFriendOfflineEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketFriendOffline,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketFriendOfflineEncoded {
    pub fn new(
        content: models::WebsocketFriendOffline,
        r#type: Type,
    ) -> WebsocketFriendOfflineEncoded {
        WebsocketFriendOfflineEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "friend-offline")]
    FriendOffline,
}

impl Default for Type {
    fn default() -> Type {
        Self::FriendOffline
    }
}
