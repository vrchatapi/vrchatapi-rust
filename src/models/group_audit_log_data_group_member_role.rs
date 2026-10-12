use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogDataGroupMemberRole {
    /// The ID of the role that was assigned or unassigned.
    #[serde(rename = "roleId")]
    pub role_id: String,
    /// The name of the role that was assigned or unassigned.
    #[serde(rename = "roleName")]
    pub role_name: String,
}

impl GroupAuditLogDataGroupMemberRole {
    pub fn new(role_id: String, role_name: String) -> GroupAuditLogDataGroupMemberRole {
        GroupAuditLogDataGroupMemberRole { role_id, role_name }
    }
}
