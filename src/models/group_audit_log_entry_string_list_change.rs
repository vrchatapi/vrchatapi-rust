use crate::models;
use serde::{Deserialize, Serialize};

/// GroupAuditLogEntryStringListChange : A list field's value before and after an update.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryStringListChange {
    #[serde(rename = "new")]
    pub new: Vec<String>,
    #[serde(rename = "old")]
    pub old: Vec<String>,
}

impl GroupAuditLogEntryStringListChange {
    /// A list field's value before and after an update.
    pub fn new(new: Vec<String>, old: Vec<String>) -> GroupAuditLogEntryStringListChange {
        GroupAuditLogEntryStringListChange { new, old }
    }
}
