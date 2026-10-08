use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BetaRegistrationNotFoundError {
    #[serde(rename = "error")]
    pub error: String,
}

impl BetaRegistrationNotFoundError {
    pub fn new(error: String) -> BetaRegistrationNotFoundError {
        BetaRegistrationNotFoundError { error }
    }
}
