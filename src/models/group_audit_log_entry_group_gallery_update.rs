use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryGroupGalleryUpdate {
    #[serde(flatten)]
    pub all_of_0: models::GroupAuditLogEntryBase,
    #[serde(flatten)]
    pub GroupAuditLogEntryGroupGalleryUpdate: serde_json::Value,
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
