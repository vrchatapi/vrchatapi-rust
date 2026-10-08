use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketFriendDeleteEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketFriendDelete,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketFriendDeleteEncoded {
    pub fn new(
        content: models::WebsocketFriendDelete,
        r#type: Type,
    ) -> WebsocketFriendDeleteEncoded {
        WebsocketFriendDeleteEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "friend-delete")]
    FriendDelete,
}

impl Default for Type {
    fn default() -> Type {
        Self::FriendDelete
    }
}
