use crate::models;
use serde::{Deserialize, Serialize};

/// WebsocketMessageUnknown : A websocket message whose `type` has no schema of its own.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketMessageUnknown {
    #[serde(
        rename = "content",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub content: Option<Option<serde_json::Value>>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl WebsocketMessageUnknown {
    /// A websocket message whose `type` has no schema of its own.
    pub fn new(r#type: String) -> WebsocketMessageUnknown {
        WebsocketMessageUnknown {
            content: None,
            r#type,
        }
    }
}
