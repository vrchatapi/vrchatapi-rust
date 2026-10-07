use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupMemberRemove {
    #[serde(flatten)]
    pub all_of_0: models::GroupAuditLogEntryBase,
    #[serde(flatten)]
    pub GroupAuditLogEntryGroupMemberRemove: serde_json::Value,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "group.member.remove")]
    GroupMemberRemove,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupMemberRemove
    }
}
