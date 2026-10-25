use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    /// The address to send the password recovery email to.
    #[serde(rename = "email")]
    pub email: String,
}

impl ResetPasswordRequest {
    pub fn new(email: String) -> ResetPasswordRequest {
        ResetPasswordRequest { email }
    }
}
