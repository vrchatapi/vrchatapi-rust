use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketFriendOnlineEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketFriendOnline,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketFriendOnlineEncoded {
    pub fn new(
        content: models::WebsocketFriendOnline,
        r#type: Type,
    ) -> WebsocketFriendOnlineEncoded {
        WebsocketFriendOnlineEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "friend-online")]
    FriendOnline,
}

impl Default for Type {
    fn default() -> Type {
        Self::FriendOnline
    }
}
