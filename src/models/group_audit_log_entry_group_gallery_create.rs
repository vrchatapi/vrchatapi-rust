use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupGalleryCreate {
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogEntryDataGroupGalleryCreate,
    #[serde(rename = "eventType")]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupGalleryCreate {
    pub fn new(
        data: models::GroupAuditLogEntryDataGroupGalleryCreate,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogEntryGroupGalleryCreate {
        GroupAuditLogEntryGroupGalleryCreate {
            data,
            event_type,
            target_id,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "group.gallery.create")]
    GroupGalleryCreate,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupGalleryCreate
    }
}
