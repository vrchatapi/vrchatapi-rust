use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupMemberSearchResponse {
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<models::GroupMember>>,
    /// Number of members returned
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

impl GroupMemberSearchResponse {
    pub fn new() -> GroupMemberSearchResponse {
        GroupMemberSearchResponse {
            results: None,
            total: None,
        }
    }
}
