use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupMemberRole {
    /// The ID of the role that was assigned or unassigned.
    #[serde(rename = "roleId")]
    pub role_id: String,
    /// The name of the role that was assigned or unassigned.
    #[serde(rename = "roleName")]
    pub role_name: String,
}

impl GroupAuditLogEntryDataGroupMemberRole {
    pub fn new(role_id: String, role_name: String) -> GroupAuditLogEntryDataGroupMemberRole {
        GroupAuditLogEntryDataGroupMemberRole { role_id, role_name }
    }
}
