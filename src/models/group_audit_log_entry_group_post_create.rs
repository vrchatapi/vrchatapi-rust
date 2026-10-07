use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupPostCreate {
    #[serde(flatten)]
    pub all_of_0: models::GroupAuditLogEntryBase,
    #[serde(flatten)]
    pub GroupAuditLogEntryGroupPostCreate: serde_json::Value,
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
