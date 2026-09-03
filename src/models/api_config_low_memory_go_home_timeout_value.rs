use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiConfigLowMemoryGoHomeTimeoutValue {
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

impl ApiConfigLowMemoryGoHomeTimeoutValue {
    pub fn new() -> ApiConfigLowMemoryGoHomeTimeoutValue {
        ApiConfigLowMemoryGoHomeTimeoutValue { timeout: None }
    }
}
