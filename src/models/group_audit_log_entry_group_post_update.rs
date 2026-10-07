use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupPostUpdate {
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogEntryDataGroupPostUpdate,
    #[serde(rename = "eventType")]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupPostUpdate {
    pub fn new(
        data: models::GroupAuditLogEntryDataGroupPostUpdate,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogEntryGroupPostUpdate {
        GroupAuditLogEntryGroupPostUpdate {
            data,
            event_type,
            target_id,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "group.post.update")]
    GroupPostUpdate,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupPostUpdate
    }
}
