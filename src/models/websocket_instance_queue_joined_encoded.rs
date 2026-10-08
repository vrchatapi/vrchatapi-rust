use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketInstanceQueueJoinedEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketInstanceQueueJoined,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketInstanceQueueJoinedEncoded {
    pub fn new(
        content: models::WebsocketInstanceQueueJoined,
        r#type: Type,
    ) -> WebsocketInstanceQueueJoinedEncoded {
        WebsocketInstanceQueueJoinedEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "instance-queue-joined")]
    InstanceQueueJoined,
}

impl Default for Type {
    fn default() -> Type {
        Self::InstanceQueueJoined
    }
}
