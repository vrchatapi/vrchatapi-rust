use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VerifyAuthTokenResult {
    #[serde(rename = "ok")]
    pub ok: bool,
    #[serde(rename = "token")]
    pub token: String,
}

impl VerifyAuthTokenResult {
    pub fn new(ok: bool, token: String) -> VerifyAuthTokenResult {
        VerifyAuthTokenResult { ok, token }
    }
}
