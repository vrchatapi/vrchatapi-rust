use crate::models;
use serde::{Deserialize, Serialize};

/// GroupAuditLogEntry : A group audit log entry. The shape of `data` depends on `eventType`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntry {
    #[serde(flatten)]
    pub all_of_0: models::GroupAuditLogEntryBase,
    #[serde(flatten)]
    pub all_of_1: models::GroupAuditLogEntryEvent,
}
