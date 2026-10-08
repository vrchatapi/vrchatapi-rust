use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupGalleryDelete {
    #[serde(flatten)]
    pub group_audit_log_entry_data_group_gallery_create:
        models::GroupAuditLogEntryDataGroupGalleryCreate,
    /// The creation timestamp of the gallery.
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// The last update timestamp of the gallery.
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl GroupAuditLogEntryDataGroupGalleryDelete {
    pub fn new(
        group_audit_log_entry_data_group_gallery_create: models::GroupAuditLogEntryDataGroupGalleryCreate,
        created_at: chrono::DateTime<chrono::FixedOffset>,
        updated_at: chrono::DateTime<chrono::FixedOffset>,
    ) -> GroupAuditLogEntryDataGroupGalleryDelete {
        GroupAuditLogEntryDataGroupGalleryDelete {
            group_audit_log_entry_data_group_gallery_create,
            created_at,
            updated_at,
        }
    }
}
