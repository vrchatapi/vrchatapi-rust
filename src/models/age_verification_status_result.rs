use crate::models;
use serde::{Deserialize, Serialize};

/// AgeVerificationStatusResult : The caller's age verification status.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgeVerificationStatusResult {
    #[serde(rename = "status")]
    pub status: String,
}

impl AgeVerificationStatusResult {
    /// The caller's age verification status.
    pub fn new(status: String) -> AgeVerificationStatusResult {
        AgeVerificationStatusResult { status }
    }
}
