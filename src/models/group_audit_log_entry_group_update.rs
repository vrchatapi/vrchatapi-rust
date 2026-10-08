use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupUpdate {
    #[serde(flatten)]
    pub group_audit_log_entry_base: models::GroupAuditLogEntryBase,
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogEntryDataGroupUpdate,
    #[serde(rename = "eventType", default)]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupUpdate {
    pub fn new(
        group_audit_log_entry_base: models::GroupAuditLogEntryBase,
        data: models::GroupAuditLogEntryDataGroupUpdate,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogEntryGroupUpdate {
        GroupAuditLogEntryGroupUpdate {
            group_audit_log_entry_base,
            data,
            event_type,
            target_id,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "group.update")]
    GroupUpdate,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupUpdate
    }
}
