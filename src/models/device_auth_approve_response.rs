use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceAuthApproveResponse {
    #[serde(rename = "success")]
    pub success: bool,
}

impl DeviceAuthApproveResponse {
    pub fn new(success: bool) -> DeviceAuthApproveResponse {
        DeviceAuthApproveResponse { success }
    }
}
