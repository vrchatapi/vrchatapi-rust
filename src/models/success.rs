use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Success {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<models::Response>,
}

impl Success {
    pub fn new() -> Success {
        Success { success: None }
    }
}
