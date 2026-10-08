use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupRoleDelete {
    #[serde(flatten)]
    pub group_audit_log_entry_base: models::GroupAuditLogEntryBase,
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogEntryDataGroupRoleDelete,
    #[serde(rename = "eventType", default)]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupRoleDelete {
    pub fn new(
        group_audit_log_entry_base: models::GroupAuditLogEntryBase,
        data: models::GroupAuditLogEntryDataGroupRoleDelete,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogEntryGroupRoleDelete {
        GroupAuditLogEntryGroupRoleDelete {
            group_audit_log_entry_base,
            data,
            event_type,
            target_id,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "group.role.delete")]
    GroupRoleDelete,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupRoleDelete
    }
}
