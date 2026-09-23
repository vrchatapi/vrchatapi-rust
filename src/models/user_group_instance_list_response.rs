use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserGroupInstanceListResponse {
    #[serde(rename = "fetchedAt", skip_serializing_if = "Option::is_none")]
    pub fetched_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[serde(rename = "instances", skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<models::Instance>>,
}

impl UserGroupInstanceListResponse {
    pub fn new() -> UserGroupInstanceListResponse {
        UserGroupInstanceListResponse {
            fetched_at: None,
            instances: None,
        }
    }
}
