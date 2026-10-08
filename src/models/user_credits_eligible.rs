use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCreditsEligible {
    #[serde(rename = "eligible")]
    pub eligible: bool,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl UserCreditsEligible {
    pub fn new(eligible: bool) -> UserCreditsEligible {
        UserCreditsEligible {
            eligible,
            reason: None,
        }
    }
}
