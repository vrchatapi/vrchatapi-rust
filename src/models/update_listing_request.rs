use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateListingRequest {
    #[serde(rename = "active")]
    pub active: bool,
}

impl UpdateListingRequest {
    pub fn new(active: bool) -> UpdateListingRequest {
        UpdateListingRequest { active }
    }
}
