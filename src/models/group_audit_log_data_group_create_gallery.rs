use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogDataGroupCreateGallery {
    #[serde(flatten)]
    pub group_audit_log_data_group_gallery_create: models::GroupAuditLogDataGroupGalleryCreate,
    #[serde(rename = "_created_at")]
    pub _created_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "_updated_at")]
    pub _updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl GroupAuditLogDataGroupCreateGallery {
    pub fn new(
        group_audit_log_data_group_gallery_create: models::GroupAuditLogDataGroupGalleryCreate,
        _created_at: chrono::DateTime<chrono::FixedOffset>,
        _id: String,
        _updated_at: chrono::DateTime<chrono::FixedOffset>,
    ) -> GroupAuditLogDataGroupCreateGallery {
        GroupAuditLogDataGroupCreateGallery {
            group_audit_log_data_group_gallery_create,
            _created_at,
            _id,
            _updated_at,
        }
    }
}
