use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketUserBadgeAssignedEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketUserBadgeAssigned,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketUserBadgeAssignedEncoded {
    pub fn new(
        content: models::WebsocketUserBadgeAssigned,
        r#type: Type,
    ) -> WebsocketUserBadgeAssignedEncoded {
        WebsocketUserBadgeAssignedEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "user-badge-assigned")]
    UserBadgeAssigned,
}

impl Default for Type {
    fn default() -> Type {
        Self::UserBadgeAssigned
    }
}
