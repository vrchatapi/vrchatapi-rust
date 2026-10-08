use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileUploadUrl {
    #[serde(rename = "url")]
    pub url: String,
}

impl FileUploadUrl {
    pub fn new(url: String) -> FileUploadUrl {
        FileUploadUrl { url }
    }
}
