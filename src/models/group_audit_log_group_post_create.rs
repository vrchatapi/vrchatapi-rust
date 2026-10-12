use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogGroupPostCreate {
    #[serde(flatten)]
    pub group_audit_log_base: models::GroupAuditLogBase,
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogDataGroupPostCreate,
    #[serde(rename = "eventType", default)]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogGroupPostCreate {
    pub fn new(
        group_audit_log_base: models::GroupAuditLogBase,
        data: models::GroupAuditLogDataGroupPostCreate,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogGroupPostCreate {
        GroupAuditLogGroupPostCreate {
            group_audit_log_base,
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
