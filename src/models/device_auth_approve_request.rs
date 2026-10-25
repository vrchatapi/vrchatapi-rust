use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceAuthApproveRequest {
    /// The `userCode` of the device code to approve.
    #[serde(rename = "userCode")]
    pub user_code: String,
}

impl DeviceAuthApproveRequest {
    pub fn new(user_code: String) -> DeviceAuthApproveRequest {
        DeviceAuthApproveRequest { user_code }
    }
}
