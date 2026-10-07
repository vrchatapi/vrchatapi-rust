use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupPostCreate {
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogEntryDataGroupPostCreate,
    #[serde(rename = "eventType")]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupPostCreate {
    pub fn new(
        data: models::GroupAuditLogEntryDataGroupPostCreate,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogEntryGroupPostCreate {
        GroupAuditLogEntryGroupPostCreate {
            data,
            event_type,
            target_id,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "group.post.create")]
    GroupPostCreate,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupPostCreate
    }
}
