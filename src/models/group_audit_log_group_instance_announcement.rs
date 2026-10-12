use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogGroupInstanceAnnouncement {
    #[serde(flatten)]
    pub group_audit_log_base: models::GroupAuditLogBase,
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogDataGroupInstanceAnnouncement,
    #[serde(rename = "eventType", default)]
    pub event_type: EventType,
    /// Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list.
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogGroupInstanceAnnouncement {
    pub fn new(
        group_audit_log_base: models::GroupAuditLogBase,
        data: models::GroupAuditLogDataGroupInstanceAnnouncement,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogGroupInstanceAnnouncement {
        GroupAuditLogGroupInstanceAnnouncement {
            group_audit_log_base,
            data,
            event_type,
            target_id,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "group.instance.announcement")]
    GroupInstanceAnnouncement,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupInstanceAnnouncement
    }
}
