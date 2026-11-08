use crate::models;
use serde::{Deserialize, Serialize};

/// A websocket message, which indicates that some sort of error has occurred
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketMessageError {
    #[serde(rename = "err")]
    pub err: String,
}

impl WebsocketMessageError {
    /// A websocket message, which indicates that some sort of error has occurred
    pub fn new(err: String) -> WebsocketMessageError {
        WebsocketMessageError { err }
    }
}
