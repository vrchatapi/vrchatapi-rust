use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupCalendarEventDelete {
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogEntryDataGroupCalendarEventDelete,
    #[serde(rename = "eventType")]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupCalendarEventDelete {
    pub fn new(
        data: models::GroupAuditLogEntryDataGroupCalendarEventDelete,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogEntryGroupCalendarEventDelete {
        GroupAuditLogEntryGroupCalendarEventDelete {
            data,
            event_type,
            target_id,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "group.calendarEvent.delete")]
    GroupCalendarEventDelete,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupCalendarEventDelete
    }
}
