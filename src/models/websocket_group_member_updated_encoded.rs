use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketGroupMemberUpdatedEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketGroupMemberUpdated,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketGroupMemberUpdatedEncoded {
    pub fn new(
        content: models::WebsocketGroupMemberUpdated,
        r#type: Type,
    ) -> WebsocketGroupMemberUpdatedEncoded {
        WebsocketGroupMemberUpdatedEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "group-member-updated")]
    GroupMemberUpdated,
}

impl Default for Type {
    fn default() -> Type {
        Self::GroupMemberUpdated
    }
}
