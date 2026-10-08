use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketClearNotification {
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketClearNotification {
    pub fn new(r#type: Type) -> WebsocketClearNotification {
        WebsocketClearNotification { r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "clear-notification")]
    ClearNotification,
}

impl Default for Type {
    fn default() -> Type {
        Self::ClearNotification
    }
}
