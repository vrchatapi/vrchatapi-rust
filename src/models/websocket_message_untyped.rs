use crate::models;
use serde::{Deserialize, Serialize};

/// A websocket message whose `type` is unknown or missing
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebsocketMessageUntyped {
    AnyOf0(models::WebsocketMessageError),
    WebsocketMessageUnknown(models::WebsocketMessageUnknown),
}

impl Default for WebsocketMessageUntyped {
    fn default() -> Self {
        Self::AnyOf0(Default::default())
    }
}

impl From<models::WebsocketMessageUnknown> for WebsocketMessageUntyped {
    fn from(value: models::WebsocketMessageUnknown) -> Self {
        Self::WebsocketMessageUnknown(value)
    }
}

impl TryFrom<WebsocketMessageUntyped> for models::WebsocketMessageUnknown {
    type Error = WebsocketMessageUntyped;

    fn try_from(value: WebsocketMessageUntyped) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessageUntyped::WebsocketMessageUnknown(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
