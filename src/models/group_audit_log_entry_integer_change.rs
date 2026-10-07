use crate::models;
use serde::{Deserialize, Serialize};

/// GroupAuditLogEntryIntegerChange : An integer field's value before and after an update.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryIntegerChange {
    #[serde(rename = "new")]
    pub new: i32,
    #[serde(rename = "old")]
    pub old: i32,
}

impl GroupAuditLogEntryIntegerChange {
    /// An integer field's value before and after an update.
    pub fn new(new: i32, old: i32) -> GroupAuditLogEntryIntegerChange {
        GroupAuditLogEntryIntegerChange { new, old }
    }
}
