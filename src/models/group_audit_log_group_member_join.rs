use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogGroupMemberJoin {
    #[serde(flatten)]
    pub group_audit_log_base: models::GroupAuditLogBase,
    #[serde(rename = "data")]
    pub data: serde_json::Value,
    #[serde(rename = "eventType", default)]
    pub event_type: EventType,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogGroupMemberJoin {
    pub fn new(
        group_audit_log_base: models::GroupAuditLogBase,
        data: serde_json::Value,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogGroupMemberJoin {
        GroupAuditLogGroupMemberJoin {
            group_audit_log_base,
            data,
            event_type,
            target_id,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "group.member.join")]
    GroupMemberJoin,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupMemberJoin
    }
}
