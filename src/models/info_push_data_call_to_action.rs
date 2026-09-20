use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataCallToAction {
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl InfoPushDataCallToAction {
    pub fn new() -> InfoPushDataCallToAction {
        InfoPushDataCallToAction { text: None }
    }
}
