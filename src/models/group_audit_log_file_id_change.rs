use crate::models;
use serde::{Deserialize, Serialize};

/// A File ID field's value before and after an update.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogFileIdChange {
    #[serde(rename = "new")]
    pub new: String,
    #[serde(rename = "old")]
    pub old: String,
}

impl GroupAuditLogFileIdChange {
    /// A File ID field's value before and after an update.
    pub fn new(new: String, old: String) -> GroupAuditLogFileIdChange {
        GroupAuditLogFileIdChange { new, old }
    }
}
