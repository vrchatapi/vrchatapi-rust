use crate::models;
use serde::{Deserialize, Serialize};

/// OkStatus : A status response consisting of solely a string description of whether the result of an operation was ok.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OkStatus {
    /// The actual status itself
    #[serde(rename = "ok")]
    pub ok: String,
}

impl OkStatus {
    /// A status response consisting of solely a string description of whether the result of an operation was ok.
    pub fn new(ok: String) -> OkStatus {
        OkStatus { ok }
    }
}
