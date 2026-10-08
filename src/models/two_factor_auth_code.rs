use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TwoFactorAuthCode {
    #[serde(rename = "code")]
    pub code: String,
}

impl TwoFactorAuthCode {
    pub fn new(code: String) -> TwoFactorAuthCode {
        TwoFactorAuthCode { code }
    }
}
