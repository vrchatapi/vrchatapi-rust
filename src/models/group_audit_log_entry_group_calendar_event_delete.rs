use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupCalendarEventDelete {
    #[serde(flatten)]
    pub group_audit_log_entry_base: models::GroupAuditLogEntryBase,
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogEntryDataGroupCalendarEventDelete,
    #[serde(rename = "eventType", default)]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupCalendarEventDelete {
    pub fn new(
        group_audit_log_entry_base: models::GroupAuditLogEntryBase,
        data: models::GroupAuditLogEntryDataGroupCalendarEventDelete,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogEntryGroupCalendarEventDelete {
        GroupAuditLogEntryGroupCalendarEventDelete {
            group_audit_log_entry_base,
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
