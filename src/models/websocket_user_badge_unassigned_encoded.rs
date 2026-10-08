use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketUserBadgeUnassignedEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketUserBadgeUnassigned,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketUserBadgeUnassignedEncoded {
    pub fn new(
        content: models::WebsocketUserBadgeUnassigned,
        r#type: Type,
    ) -> WebsocketUserBadgeUnassignedEncoded {
        WebsocketUserBadgeUnassignedEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "user-badge-unassigned")]
    UserBadgeUnassigned,
}

impl Default for Type {
    fn default() -> Type {
        Self::UserBadgeUnassigned
    }
}
