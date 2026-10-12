use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedGroupAuditLogList {
    /// Whether there are more results after this page.
    #[serde(rename = "hasNext", skip_serializing_if = "Option::is_none")]
    pub has_next: Option<bool>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<models::GroupAuditLog>>,
    /// The total number of results that the query would return if there were no pagination.
    #[serde(rename = "totalCount", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

impl PaginatedGroupAuditLogList {
    pub fn new() -> PaginatedGroupAuditLogList {
        PaginatedGroupAuditLogList {
            has_next: None,
            results: None,
            total_count: None,
        }
    }
}
