use crate::models;
use serde::{Deserialize, Serialize};

/// A File ID field's value before and after an update.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryFileIdChange {
    #[serde(rename = "new")]
    pub new: String,
    #[serde(rename = "old")]
    pub old: String,
}

impl GroupAuditLogEntryFileIdChange {
    /// A File ID field's value before and after an update.
    pub fn new(new: String, old: String) -> GroupAuditLogEntryFileIdChange {
        GroupAuditLogEntryFileIdChange { new, old }
    }
}
