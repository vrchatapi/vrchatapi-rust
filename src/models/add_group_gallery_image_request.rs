use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddGroupGalleryImageRequest {
    #[serde(rename = "fileId")]
    pub file_id: String,
}

impl AddGroupGalleryImageRequest {
    pub fn new(file_id: String) -> AddGroupGalleryImageRequest {
        AddGroupGalleryImageRequest { file_id }
    }
}
