use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketModifiedImageUpdate {
    #[serde(rename = "fileId")]
    pub file_id: String,
    #[serde(rename = "needsProcessing")]
    pub needs_processing: bool,
    #[serde(rename = "pixelSize")]
    pub pixel_size: f64,
    #[serde(rename = "versionNumber")]
    pub version_number: f64,
}

impl WebsocketModifiedImageUpdate {
    pub fn new(
        file_id: String,
        needs_processing: bool,
        pixel_size: f64,
        version_number: f64,
    ) -> WebsocketModifiedImageUpdate {
        WebsocketModifiedImageUpdate {
            file_id,
            needs_processing,
            pixel_size,
            version_number,
        }
    }
}
