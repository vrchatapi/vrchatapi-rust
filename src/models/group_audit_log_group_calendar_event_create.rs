use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogGroupCalendarEventCreate {
    #[serde(flatten)]
    pub group_audit_log_base: models::GroupAuditLogBase,
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogDataGroupCalendarEventCreate,
    #[serde(rename = "eventType", default)]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogGroupCalendarEventCreate {
    pub fn new(
        group_audit_log_base: models::GroupAuditLogBase,
        data: models::GroupAuditLogDataGroupCalendarEventCreate,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogGroupCalendarEventCreate {
        GroupAuditLogGroupCalendarEventCreate {
            group_audit_log_base,
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
