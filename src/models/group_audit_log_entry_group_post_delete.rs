use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupPostDelete {
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogEntryDataGroupPostDelete,
    #[serde(rename = "eventType")]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupPostDelete {
    pub fn new(
        data: models::GroupAuditLogEntryDataGroupPostDelete,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogEntryGroupPostDelete {
        GroupAuditLogEntryGroupPostDelete {
            data,
            event_type,
            target_id,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "group.post.delete")]
    GroupPostDelete,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupPostDelete
    }
}
