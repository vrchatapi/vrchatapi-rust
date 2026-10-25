use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckDeviceCodeRequest {
    #[serde(rename = "deviceCode")]
    pub device_code: String,
}

impl CheckDeviceCodeRequest {
    pub fn new(device_code: String) -> CheckDeviceCodeRequest {
        CheckDeviceCodeRequest { device_code }
    }
}
