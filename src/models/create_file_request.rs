use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFileRequest {
    #[serde(rename = "extension")]
    pub extension: String,
    #[serde(rename = "mimeType")]
    pub mime_type: models::CreateFileRequestMimeType,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl CreateFileRequest {
    pub fn new(
        extension: String,
        mime_type: models::CreateFileRequestMimeType,
        name: String,
    ) -> CreateFileRequest {
        CreateFileRequest {
            extension,
            mime_type,
            name,
            tags: None,
        }
    }
}
