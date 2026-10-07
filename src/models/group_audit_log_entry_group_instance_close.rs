use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupInstanceClose {
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogEntryDataGroupInstanceClose,
    #[serde(rename = "eventType")]
    pub event_type: EventType,
    /// Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list.
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupInstanceClose {
    pub fn new(
        data: models::GroupAuditLogEntryDataGroupInstanceClose,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogEntryGroupInstanceClose {
        GroupAuditLogEntryGroupInstanceClose {
            data,
            event_type,
            target_id,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "group.instance.close")]
    GroupInstanceClose,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupInstanceClose
    }
}
