use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupGalleryUpdate {
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogEntryDataGroupGalleryUpdate,
    #[serde(rename = "eventType")]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupGalleryUpdate {
    pub fn new(
        data: models::GroupAuditLogEntryDataGroupGalleryUpdate,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogEntryGroupGalleryUpdate {
        GroupAuditLogEntryGroupGalleryUpdate {
            data,
            event_type,
            target_id,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "group.gallery.update")]
    GroupGalleryUpdate,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupGalleryUpdate
    }
}
