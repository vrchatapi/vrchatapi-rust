/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// FileData :
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileData {
    #[serde(rename = "category")]
    pub category: Category,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "md5", skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    #[serde(rename = "sizeInBytes")]
    pub size_in_bytes: i32,
    #[serde(rename = "status")]
    pub status: models::FileStatus,
    #[serde(rename = "uploadId")]
    pub upload_id: String,
    #[serde(rename = "url")]
    pub url: String,
}

impl FileData {
    pub fn new(
        category: Category,
        file_name: String,
        size_in_bytes: i32,
        status: models::FileStatus,
        upload_id: String,
        url: String,
    ) -> FileData {
        FileData {
            category,
            file_name,
            md5: None,
            size_in_bytes,
            status,
            upload_id,
            url,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "multipart")]
    Multipart,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "simple")]
    Simple,
}

impl Default for Category {
    fn default() -> Category {
        Self::Multipart
    }
}
