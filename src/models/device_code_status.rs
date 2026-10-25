use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCodeStatus {
    #[serde(rename = "status")]
    pub status: String,
    /// Set once someone approves the code, when the response also logs this device in.
    #[serde(
        rename = "user",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<Option<serde_json::Value>>,
}

impl DeviceCodeStatus {
    pub fn new(status: String) -> DeviceCodeStatus {
        DeviceCodeStatus { status, user: None }
    }
}
