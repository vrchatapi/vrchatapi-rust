use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogDataGroupInstanceClose {
    #[serde(rename = "groupAccessType")]
    pub group_access_type: models::GroupAccessType,
    /// The role IDs that have access to the instance.
    #[serde(rename = "roleIds", deserialize_with = "Option::deserialize")]
    pub role_ids: Option<Vec<String>>,
}

impl GroupAuditLogDataGroupInstanceClose {
    pub fn new(
        group_access_type: models::GroupAccessType,
        role_ids: Option<Vec<String>>,
    ) -> GroupAuditLogDataGroupInstanceClose {
        GroupAuditLogDataGroupInstanceClose {
            group_access_type,
            role_ids,
        }
    }
}
