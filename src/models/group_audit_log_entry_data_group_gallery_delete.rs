use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupGalleryDelete {
    #[serde(flatten)]
    pub all_of_0: models::GroupAuditLogEntryDataGroupGalleryCreate,
    #[serde(flatten)]
    pub GroupAuditLogEntryDataGroupGalleryDelete: serde_json::Value,
}
