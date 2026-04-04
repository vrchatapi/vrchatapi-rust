use crate::models;
use serde::{Deserialize, Serialize};

/// CreateJamSubmissionRequest : Submit content for a Jam. Both content upload by submitter and jam submission itself must be made within the jam's designated times.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateJamSubmissionRequest {
    /// The id of the uploaded content (e.g., avatar, world) being submitted.
    #[serde(rename = "contentId")]
    pub content_id: String,
    /// A description of the content being submitted.
    #[serde(rename = "description")]
    pub description: String,
}

impl CreateJamSubmissionRequest {
    /// Submit content for a Jam. Both content upload by submitter and jam submission itself must be made within the jam's designated times.
    pub fn new(content_id: String, description: String) -> CreateJamSubmissionRequest {
        CreateJamSubmissionRequest {
            content_id,
            description,
        }
    }
}
