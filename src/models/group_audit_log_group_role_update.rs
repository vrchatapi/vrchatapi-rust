use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogGroupRoleUpdate {
    #[serde(flatten)]
    pub group_audit_log_base: models::GroupAuditLogBase,
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogDataGroupRoleUpdate,
    #[serde(rename = "eventType", default)]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogGroupRoleUpdate {
    pub fn new(
        group_audit_log_base: models::GroupAuditLogBase,
        data: models::GroupAuditLogDataGroupRoleUpdate,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogGroupRoleUpdate {
        GroupAuditLogGroupRoleUpdate {
            group_audit_log_base,
            data,
            event_type,
            target_id,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "group.role.update")]
    GroupRoleUpdate,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupRoleUpdate
    }
}
