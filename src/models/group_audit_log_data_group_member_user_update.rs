use crate::models;
use serde::{Deserialize, Serialize};

/// Carries only the fields the update changed.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogDataGroupMemberUserUpdate {
    #[serde(rename = "managerNotes", skip_serializing_if = "Option::is_none")]
    pub manager_notes: Option<models::GroupAuditLogStringChange>,
}

impl GroupAuditLogDataGroupMemberUserUpdate {
    /// Carries only the fields the update changed.
    pub fn new() -> GroupAuditLogDataGroupMemberUserUpdate {
        GroupAuditLogDataGroupMemberUserUpdate {
            manager_notes: None,
        }
    }
}
