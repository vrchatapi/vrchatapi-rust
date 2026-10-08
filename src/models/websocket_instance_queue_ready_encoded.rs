use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketInstanceQueueReadyEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketInstanceQueueReady,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketInstanceQueueReadyEncoded {
    pub fn new(
        content: models::WebsocketInstanceQueueReady,
        r#type: Type,
    ) -> WebsocketInstanceQueueReadyEncoded {
        WebsocketInstanceQueueReadyEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "instance-queue-ready")]
    InstanceQueueReady,
}

impl Default for Type {
    fn default() -> Type {
        Self::InstanceQueueReady
    }
}
