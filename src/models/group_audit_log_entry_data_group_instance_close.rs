use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupInstanceClose {
    #[serde(rename = "groupAccessType")]
    pub group_access_type: models::GroupAccessType,
    /// The role IDs that have access to the instance.
    #[serde(rename = "roleIds", deserialize_with = "Option::deserialize")]
    pub role_ids: Option<Vec<String>>,
}

impl GroupAuditLogEntryDataGroupInstanceClose {
    pub fn new(
        group_access_type: models::GroupAccessType,
        role_ids: Option<Vec<String>>,
    ) -> GroupAuditLogEntryDataGroupInstanceClose {
        GroupAuditLogEntryDataGroupInstanceClose {
            group_access_type,
            role_ids,
        }
    }
}
