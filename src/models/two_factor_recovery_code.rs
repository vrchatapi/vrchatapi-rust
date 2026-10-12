use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TwoFactorRecoveryCode {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "used")]
    pub used: bool,
}

impl TwoFactorRecoveryCode {
    pub fn new(code: String, used: bool) -> TwoFactorRecoveryCode {
        TwoFactorRecoveryCode { code, used }
    }
}
