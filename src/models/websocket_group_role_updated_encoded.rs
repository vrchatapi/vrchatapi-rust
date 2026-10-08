use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketGroupRoleUpdatedEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketGroupRoleUpdated,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketGroupRoleUpdatedEncoded {
    pub fn new(
        content: models::WebsocketGroupRoleUpdated,
        r#type: Type,
    ) -> WebsocketGroupRoleUpdatedEncoded {
        WebsocketGroupRoleUpdatedEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "group-role-updated")]
    GroupRoleUpdated,
}

impl Default for Type {
    fn default() -> Type {
        Self::GroupRoleUpdated
    }
}
