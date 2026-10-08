use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupPostDelete {
    /// The ID of the post author.
    #[serde(rename = "authorId")]
    pub author_id: String,
    /// The image file ID attached to the post.
    #[serde(rename = "imageId", deserialize_with = "Option::deserialize")]
    pub image_id: Option<String>,
    /// The text content of the post.
    #[serde(rename = "text")]
    pub text: String,
    /// The title of the post.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "visibility")]
    pub visibility: models::GroupPostVisibility,
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
        author_id: String,
        image_id: Option<String>,
        text: String,
        title: String,
        visibility: models::GroupPostVisibility,
        created_at: chrono::DateTime<chrono::FixedOffset>,
        editor_id: Option<String>,
        image_url: Option<String>,
        role_ids: Vec<String>,
        updated_at: chrono::DateTime<chrono::FixedOffset>,
    ) -> GroupAuditLogEntryDataGroupPostDelete {
        GroupAuditLogEntryDataGroupPostDelete {
            author_id,
            image_id,
            text,
            title,
            visibility,
            created_at,
            editor_id,
            image_url,
            role_ids,
            updated_at,
        }
    }
}
