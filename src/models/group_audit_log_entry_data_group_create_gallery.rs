use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupCreateGallery {
    #[serde(flatten)]
    pub group_audit_log_entry_data_group_gallery_create:
        models::GroupAuditLogEntryDataGroupGalleryCreate,
    #[serde(rename = "_created_at")]
    pub _created_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "_updated_at")]
    pub _updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl GroupAuditLogEntryDataGroupCreateGallery {
    pub fn new(
        group_audit_log_entry_data_group_gallery_create: models::GroupAuditLogEntryDataGroupGalleryCreate,
        _created_at: chrono::DateTime<chrono::FixedOffset>,
        _id: String,
        _updated_at: chrono::DateTime<chrono::FixedOffset>,
    ) -> GroupAuditLogEntryDataGroupCreateGallery {
        GroupAuditLogEntryDataGroupCreateGallery {
            group_audit_log_entry_data_group_gallery_create,
            _created_at,
            _id,
            _updated_at,
        }
    }
}
