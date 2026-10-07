use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupUpdate {
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogEntryDataGroupUpdate,
    #[serde(rename = "eventType")]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupUpdate {
    pub fn new(
        data: models::GroupAuditLogEntryDataGroupUpdate,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogEntryGroupUpdate {
        GroupAuditLogEntryGroupUpdate {
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
