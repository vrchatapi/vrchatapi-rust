use crate::models;
use serde::{Deserialize, Serialize};

/// BareError : An error body carrying only a message string. Unlike `Error`, there is no nested object and no `status_code`, so a consumer that assumes the usual shape will read `undefined` from it.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BareError {
    #[serde(rename = "error")]
    pub error: String,
}

impl BareError {
    /// An error body carrying only a message string. Unlike `Error`, there is no nested object and no `status_code`, so a consumer that assumes the usual shape will read `undefined` from it.
    pub fn new(error: String) -> BareError {
        BareError { error }
    }
}
