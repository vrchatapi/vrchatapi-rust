use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketFriendLocationEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketFriendLocation,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketFriendLocationEncoded {
    pub fn new(
        content: models::WebsocketFriendLocation,
        r#type: Type,
    ) -> WebsocketFriendLocationEncoded {
        WebsocketFriendLocationEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "friend-location")]
    FriendLocation,
}

impl Default for Type {
    fn default() -> Type {
        Self::FriendLocation
    }
}
