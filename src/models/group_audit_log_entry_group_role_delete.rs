use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupRoleDelete {
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogEntryDataGroupRoleDelete,
    #[serde(rename = "eventType")]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupRoleDelete {
    pub fn new(
        data: models::GroupAuditLogEntryDataGroupRoleDelete,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogEntryGroupRoleDelete {
        GroupAuditLogEntryGroupRoleDelete {
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
