use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupRequestReject {
    /// The display name of the user who performed the action.
    #[serde(rename = "actorDisplayName")]
    pub actor_display_name: String,
    /// The ID of the user who performed the action.
    #[serde(rename = "actorId")]
    pub actor_id: String,
    /// When the action was performed.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// A human-readable description of the event.
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "eventType", default)]
    pub event_type: EventType,
    /// The ID of the group the entry belongs to.
    #[serde(rename = "groupId")]
    pub group_id: String,
    /// The unique ID of this audit log entry.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "data")]
    pub data: serde_json::Value,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupRequestReject {
    pub fn new(
        actor_display_name: String,
        actor_id: String,
        created_at: chrono::DateTime<chrono::FixedOffset>,
        description: String,
        event_type: EventType,
        group_id: String,
        id: String,
        data: serde_json::Value,
        target_id: String,
    ) -> GroupAuditLogEntryGroupRequestReject {
        GroupAuditLogEntryGroupRequestReject {
            actor_display_name,
            actor_id,
            created_at,
            description,
            event_type,
            group_id,
            id,
            data,
            target_id,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "group.request.reject")]
    GroupRequestReject,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupRequestReject
    }
}
