use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketGroupJoinedEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketGroupJoined,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketGroupJoinedEncoded {
    pub fn new(content: models::WebsocketGroupJoined, r#type: Type) -> WebsocketGroupJoinedEncoded {
        WebsocketGroupJoinedEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "group-joined")]
    GroupJoined,
}

impl Default for Type {
    fn default() -> Type {
        Self::GroupJoined
    }
}
