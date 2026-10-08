use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketFriendUpdateEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketFriendUpdate,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketFriendUpdateEncoded {
    pub fn new(
        content: models::WebsocketFriendUpdate,
        r#type: Type,
    ) -> WebsocketFriendUpdateEncoded {
        WebsocketFriendUpdateEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "friend-update")]
    FriendUpdate,
}

impl Default for Type {
    fn default() -> Type {
        Self::FriendUpdate
    }
}
