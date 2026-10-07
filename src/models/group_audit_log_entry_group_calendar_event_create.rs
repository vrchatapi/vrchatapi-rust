use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupCalendarEventCreate {
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogEntryDataGroupCalendarEventCreate,
    #[serde(rename = "eventType")]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupCalendarEventCreate {
    pub fn new(
        data: models::GroupAuditLogEntryDataGroupCalendarEventCreate,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogEntryGroupCalendarEventCreate {
        GroupAuditLogEntryGroupCalendarEventCreate {
            data,
            event_type,
            target_id,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "group.calendarEvent.create")]
    GroupCalendarEventCreate,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupCalendarEventCreate
    }
}
