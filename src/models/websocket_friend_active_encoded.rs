use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketFriendActiveEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketFriendActive,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketFriendActiveEncoded {
    pub fn new(
        content: models::WebsocketFriendActive,
        r#type: Type,
    ) -> WebsocketFriendActiveEncoded {
        WebsocketFriendActiveEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "friend-active")]
    FriendActive,
}

impl Default for Type {
    fn default() -> Type {
        Self::FriendActive
    }
}
