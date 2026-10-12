use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogDataGroupGalleryDelete {
    #[serde(flatten)]
    pub group_audit_log_data_group_gallery_create: models::GroupAuditLogDataGroupGalleryCreate,
    /// The creation timestamp of the gallery.
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// The last update timestamp of the gallery.
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl GroupAuditLogDataGroupGalleryDelete {
    pub fn new(
        group_audit_log_data_group_gallery_create: models::GroupAuditLogDataGroupGalleryCreate,
        created_at: chrono::DateTime<chrono::FixedOffset>,
        updated_at: chrono::DateTime<chrono::FixedOffset>,
    ) -> GroupAuditLogDataGroupGalleryDelete {
        GroupAuditLogDataGroupGalleryDelete {
            group_audit_log_data_group_gallery_create,
            created_at,
            updated_at,
        }
    }
}
