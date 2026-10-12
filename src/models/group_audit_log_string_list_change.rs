use crate::models;
use serde::{Deserialize, Serialize};

/// A list field's value before and after an update.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogStringListChange {
    #[serde(rename = "new")]
    pub new: Vec<String>,
    #[serde(rename = "old")]
    pub old: Vec<String>,
}

impl GroupAuditLogStringListChange {
    /// A list field's value before and after an update.
    pub fn new(new: Vec<String>, old: Vec<String>) -> GroupAuditLogStringListChange {
        GroupAuditLogStringListChange { new, old }
    }
}
