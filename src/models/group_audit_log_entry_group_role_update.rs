use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupRoleUpdate {
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogEntryDataGroupRoleUpdate,
    #[serde(rename = "eventType")]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupRoleUpdate {
    pub fn new(
        data: models::GroupAuditLogEntryDataGroupRoleUpdate,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogEntryGroupRoleUpdate {
        GroupAuditLogEntryGroupRoleUpdate {
            data,
            event_type,
            target_id,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "group.role.update")]
    GroupRoleUpdate,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupRoleUpdate
    }
}
