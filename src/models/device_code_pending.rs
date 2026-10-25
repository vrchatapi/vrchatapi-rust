use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCodePending {
    #[serde(rename = "status")]
    pub status: String,
}

impl DeviceCodePending {
    pub fn new(status: String) -> DeviceCodePending {
        DeviceCodePending { status }
    }
}
