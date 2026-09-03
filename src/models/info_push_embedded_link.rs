use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushEmbeddedLink {
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
    #[serde(rename = "command", skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl InfoPushEmbeddedLink {
    pub fn new() -> InfoPushEmbeddedLink {
        InfoPushEmbeddedLink {
            parameters: None,
            command: None,
            id: None,
            name: None,
        }
    }
}
