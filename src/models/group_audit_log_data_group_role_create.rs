use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogDataGroupRoleCreate {
    #[serde(flatten)]
    pub group_audit_log_data_group_role: models::GroupAuditLogDataGroupRole,
    /// The group ID.
    #[serde(rename = "groupId")]
    pub group_id: String,
    /// The ID of the user who last updated the role.
    #[serde(rename = "lastUpdatedByUserId")]
    pub last_updated_by_user_id: String,
}

impl GroupAuditLogDataGroupRoleCreate {
    pub fn new(
        group_audit_log_data_group_role: models::GroupAuditLogDataGroupRole,
        group_id: String,
        last_updated_by_user_id: String,
    ) -> GroupAuditLogDataGroupRoleCreate {
        GroupAuditLogDataGroupRoleCreate {
            group_audit_log_data_group_role,
            group_id,
            last_updated_by_user_id,
        }
    }
}
