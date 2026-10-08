use crate::models;
use serde::{Deserialize, Serialize};

/// GroupAuditLogEntryJoinStateChange : A join state field's value before and after an update.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryJoinStateChange {
    #[serde(rename = "new")]
    pub new: models::GroupJoinState,
    #[serde(rename = "old")]
    pub old: models::GroupJoinState,
}

impl GroupAuditLogEntryJoinStateChange {
    /// A join state field's value before and after an update.
    pub fn new(
        new: models::GroupJoinState,
        old: models::GroupJoinState,
    ) -> GroupAuditLogEntryJoinStateChange {
        GroupAuditLogEntryJoinStateChange { new, old }
    }
}
