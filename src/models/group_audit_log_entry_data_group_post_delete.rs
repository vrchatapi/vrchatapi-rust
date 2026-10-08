use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupPostDelete {
    #[serde(flatten)]
    pub group_audit_log_entry_data_group_post: models::GroupAuditLogEntryDataGroupPost,
    /// The creation timestamp of the post.
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// The ID of the user who last edited the post.
    #[serde(rename = "editorId", deserialize_with = "Option::deserialize")]
    pub editor_id: Option<String>,
    /// The URL of the post image.
    #[serde(rename = "imageUrl", deserialize_with = "Option::deserialize")]
    pub image_url: Option<String>,
    /// The role IDs that could see the post.
    #[serde(rename = "roleIds")]
    pub role_ids: Vec<String>,
    /// The last update timestamp of the post.
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl GroupAuditLogEntryDataGroupPostDelete {
    pub fn new(
        group_audit_log_entry_data_group_post: models::GroupAuditLogEntryDataGroupPost,
        created_at: chrono::DateTime<chrono::FixedOffset>,
        editor_id: Option<String>,
        image_url: Option<String>,
        role_ids: Vec<String>,
        updated_at: chrono::DateTime<chrono::FixedOffset>,
    ) -> GroupAuditLogEntryDataGroupPostDelete {
        GroupAuditLogEntryDataGroupPostDelete {
            group_audit_log_entry_data_group_post,
            created_at,
            editor_id,
            image_url,
            role_ids,
            updated_at,
        }
    }
}
