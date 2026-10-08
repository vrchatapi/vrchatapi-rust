use crate::models;
use serde::{Deserialize, Serialize};

/// GroupAuditLogEntryStringChange : A text field's value before and after an update.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryStringChange {
    #[serde(rename = "new")]
    pub new: String,
    #[serde(rename = "old")]
    pub old: String,
}

impl GroupAuditLogEntryStringChange {
    /// A text field's value before and after an update.
    pub fn new(new: String, old: String) -> GroupAuditLogEntryStringChange {
        GroupAuditLogEntryStringChange { new, old }
    }
}
