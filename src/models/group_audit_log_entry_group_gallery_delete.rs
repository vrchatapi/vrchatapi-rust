use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupGalleryDelete {
    #[serde(rename = "data")]
    pub data: models::GroupAuditLogEntryDataGroupGalleryDelete,
    #[serde(rename = "eventType")]
    pub event_type: EventType,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryGroupGalleryDelete {
    pub fn new(
        data: models::GroupAuditLogEntryDataGroupGalleryDelete,
        event_type: EventType,
        target_id: String,
    ) -> GroupAuditLogEntryGroupGalleryDelete {
        GroupAuditLogEntryGroupGalleryDelete {
            data,
            event_type,
            target_id,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "group.gallery.delete")]
    GroupGalleryDelete,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::GroupGalleryDelete
    }
}
