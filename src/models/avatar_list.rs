use crate::models;
use serde::{Deserialize, Serialize};

/// A page of avatar search results.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvatarList {
    #[serde(rename = "hasNext")]
    pub has_next: bool,
    #[serde(rename = "results")]
    pub results: Vec<models::Avatar>,
    #[serde(rename = "totalCount")]
    pub total_count: i32,
}

impl AvatarList {
    /// A page of avatar search results.
    pub fn new(has_next: bool, results: Vec<models::Avatar>, total_count: i32) -> AvatarList {
        AvatarList {
            has_next,
            results,
            total_count,
        }
    }
}
