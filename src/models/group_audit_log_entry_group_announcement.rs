use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupAnnouncement {
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogEntryDataGroupAnnouncement,
    #[serde(rename = "eventType")]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupAnnouncement {
    pub fn new(
        data: models::GroupAuditLogEntryDataGroupAnnouncement,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogEntryGroupAnnouncement {
        GroupAuditLogEntryGroupAnnouncement {
            data,
            event_type,
            target_id,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "group.announcement")]
    GroupAnnouncement,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupAnnouncement
    }
}
