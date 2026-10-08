use crate::models;
use serde::{Deserialize, Serialize};

/// GroupAuditLogEntryBooleanChange : A boolean field's value before and after an update.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryBooleanChange {
    #[serde(rename = "new")]
    pub new: bool,
    #[serde(rename = "old")]
    pub old: bool,
}

impl GroupAuditLogEntryBooleanChange {
    /// A boolean field's value before and after an update.
    pub fn new(new: bool, old: bool) -> GroupAuditLogEntryBooleanChange {
        GroupAuditLogEntryBooleanChange { new, old }
    }
}
