use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
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
