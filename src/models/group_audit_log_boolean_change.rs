use crate::models;
use serde::{Deserialize, Serialize};

/// A boolean field's value before and after an update.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogBooleanChange {
    #[serde(rename = "new")]
    pub new: bool,
    #[serde(rename = "old")]
    pub old: bool,
}

impl GroupAuditLogBooleanChange {
    /// A boolean field's value before and after an update.
    pub fn new(new: bool, old: bool) -> GroupAuditLogBooleanChange {
        GroupAuditLogBooleanChange { new, old }
    }
}
