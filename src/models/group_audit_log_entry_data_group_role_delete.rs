use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupRoleDelete {
    #[serde(flatten)]
    pub group_audit_log_entry_data_group_role: models::GroupAuditLogEntryDataGroupRole,
    /// The creation timestamp of the role.
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// Whether the role is the group's default role.
    #[serde(rename = "defaultRole")]
    pub default_role: bool,
    /// Whether the role is a management role.
    #[serde(rename = "isManagementRole")]
    pub is_management_role: bool,
}

impl GroupAuditLogEntryDataGroupRoleDelete {
    pub fn new(
        group_audit_log_entry_data_group_role: models::GroupAuditLogEntryDataGroupRole,
        created_at: chrono::DateTime<chrono::FixedOffset>,
        default_role: bool,
        is_management_role: bool,
    ) -> GroupAuditLogEntryDataGroupRoleDelete {
        GroupAuditLogEntryDataGroupRoleDelete {
            group_audit_log_entry_data_group_role,
            created_at,
            default_role,
            is_management_role,
        }
    }
}
