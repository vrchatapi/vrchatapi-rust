use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupPost {
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
}

impl GroupAuditLogEntryDataGroupPost {
    pub fn new(
        author_id: String,
        image_id: Option<String>,
        text: String,
        title: String,
        visibility: models::GroupPostVisibility,
    ) -> GroupAuditLogEntryDataGroupPost {
        GroupAuditLogEntryDataGroupPost {
            author_id,
            image_id,
            text,
            title,
            visibility,
        }
    }
}
