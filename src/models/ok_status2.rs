use crate::models;
use serde::{Deserialize, Serialize};

/// Another status response consisting of solely a string description of whether the result of an operation was ok.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OkStatus2 {
    /// The actual status itself
    #[serde(rename = "OK")]
    pub ok: String,
}

impl OkStatus2 {
    /// Another status response consisting of solely a string description of whether the result of an operation was ok.
    pub fn new(ok: String) -> OkStatus2 {
        OkStatus2 { ok }
    }
}
