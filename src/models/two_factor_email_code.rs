use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TwoFactorEmailCode {
    #[serde(rename = "code")]
    pub code: String,
}

impl TwoFactorEmailCode {
    pub fn new(code: String) -> TwoFactorEmailCode {
        TwoFactorEmailCode { code }
    }
}
