use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupRoleCreate {
    #[serde(flatten)]
    pub group_audit_log_entry_data_group_role: models::GroupAuditLogEntryDataGroupRole,
    /// The group ID.
    #[serde(rename = "groupId")]
    pub group_id: String,
    /// The ID of the user who last updated the role.
    #[serde(rename = "lastUpdatedByUserId")]
    pub last_updated_by_user_id: String,
}

impl GroupAuditLogEntryDataGroupRoleCreate {
    pub fn new(
        group_audit_log_entry_data_group_role: models::GroupAuditLogEntryDataGroupRole,
        group_id: String,
        last_updated_by_user_id: String,
    ) -> GroupAuditLogEntryDataGroupRoleCreate {
        GroupAuditLogEntryDataGroupRoleCreate {
            group_audit_log_entry_data_group_role,
            group_id,
            last_updated_by_user_id,
        }
    }
}
