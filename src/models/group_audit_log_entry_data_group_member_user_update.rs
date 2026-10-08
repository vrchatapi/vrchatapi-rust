use crate::models;
use serde::{Deserialize, Serialize};

/// GroupAuditLogEntryDataGroupMemberUserUpdate : Carries only the fields the update changed.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupMemberUserUpdate {
    #[serde(rename = "managerNotes", skip_serializing_if = "Option::is_none")]
    pub manager_notes: Option<models::GroupAuditLogEntryStringChange>,
}

impl GroupAuditLogEntryDataGroupMemberUserUpdate {
    /// Carries only the fields the update changed.
    pub fn new() -> GroupAuditLogEntryDataGroupMemberUserUpdate {
        GroupAuditLogEntryDataGroupMemberUserUpdate {
            manager_notes: None,
        }
    }
}
