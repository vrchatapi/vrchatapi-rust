use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupPostUpdate {
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
    pub data: models::GroupAuditLogEntryDataGroupPostUpdate,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupPostUpdate {
    pub fn new(
        actor_display_name: String,
        actor_id: String,
        created_at: chrono::DateTime<chrono::FixedOffset>,
        description: String,
        event_type: EventType,
        group_id: String,
        id: String,
        data: models::GroupAuditLogEntryDataGroupPostUpdate,
        target_id: String,
    ) -> GroupAuditLogEntryGroupPostUpdate {
        GroupAuditLogEntryGroupPostUpdate {
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
    #[serde(rename = "group.post.update")]
    GroupPostUpdate,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupPostUpdate
    }
}
