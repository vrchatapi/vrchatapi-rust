use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketInstanceClosedEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketInstanceClosed,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketInstanceClosedEncoded {
    pub fn new(
        content: models::WebsocketInstanceClosed,
        r#type: Type,
    ) -> WebsocketInstanceClosedEncoded {
        WebsocketInstanceClosedEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "instance-closed")]
    InstanceClosed,
}

impl Default for Type {
    fn default() -> Type {
        Self::InstanceClosed
    }
}
