use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateTiliaTosRequest {
    #[serde(rename = "accepted")]
    pub accepted: bool,
}

impl UpdateTiliaTosRequest {
    pub fn new(accepted: bool) -> UpdateTiliaTosRequest {
        UpdateTiliaTosRequest { accepted }
    }
}
