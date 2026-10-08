use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupRoleUpdate {
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
    /// The ID of the group the entry belongs to.
    #[serde(rename = "groupId")]
    pub group_id: String,
    /// The unique ID of this audit log entry.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogEntryDataGroupRoleUpdate,
    #[serde(rename = "eventType", default)]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupRoleUpdate {
    pub fn new(
        actor_display_name: String,
        actor_id: String,
        created_at: chrono::DateTime<chrono::FixedOffset>,
        description: String,
        group_id: String,
        id: String,
        data: models::GroupAuditLogEntryDataGroupRoleUpdate,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogEntryGroupRoleUpdate {
        GroupAuditLogEntryGroupRoleUpdate {
            actor_display_name,
            actor_id,
            created_at,
            description,
            group_id,
            id,
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
