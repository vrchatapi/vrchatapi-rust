use crate::models;
use serde::{Deserialize, Serialize};

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
