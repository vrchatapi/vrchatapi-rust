use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupRoleCreate {
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogEntryDataGroupRoleCreate,
    #[serde(rename = "eventType")]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupRoleCreate {
    pub fn new(
        data: models::GroupAuditLogEntryDataGroupRoleCreate,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogEntryGroupRoleCreate {
        GroupAuditLogEntryGroupRoleCreate {
            data,
            event_type,
            target_id,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "group.role.create")]
    GroupRoleCreate,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupRoleCreate
    }
}
