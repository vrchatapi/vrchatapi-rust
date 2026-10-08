use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TwoFactorRecoveryCodesOtpInner {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "used")]
    pub used: bool,
}

impl TwoFactorRecoveryCodesOtpInner {
    pub fn new(code: String, used: bool) -> TwoFactorRecoveryCodesOtpInner {
        TwoFactorRecoveryCodesOtpInner { code, used }
    }
}
