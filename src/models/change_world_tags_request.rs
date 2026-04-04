use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeWorldTagsRequest {
    /// The tags being added or removed.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

impl ChangeWorldTagsRequest {
    pub fn new(tags: Vec<String>) -> ChangeWorldTagsRequest {
        ChangeWorldTagsRequest { tags }
    }
}
