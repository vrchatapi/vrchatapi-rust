use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketFriendAddEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketFriendAdd,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketFriendAddEncoded {
    pub fn new(content: models::WebsocketFriendAdd, r#type: Type) -> WebsocketFriendAddEncoded {
        WebsocketFriendAddEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "friend-add")]
    FriendAdd,
}

impl Default for Type {
    fn default() -> Type {
        Self::FriendAdd
    }
}
