use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuccessFlag {
    #[serde(rename = "success")]
    pub success: bool,
}

impl SuccessFlag {
    pub fn new(success: bool) -> SuccessFlag {
        SuccessFlag { success }
    }
}
