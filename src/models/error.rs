use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<models::Response>,
}

impl Error {
    pub fn new() -> Error {
        Error { error: None }
    }
}
