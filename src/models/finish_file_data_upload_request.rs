/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// FinishFileDataUploadRequest :
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FinishFileDataUploadRequest {
    /// Array of ETags uploaded.
    #[serde(rename = "etags", skip_serializing_if = "Option::is_none")]
    pub etags: Option<Vec<String>>,
    /// Always a zero in string form, despite how many parts uploaded.
    #[serde(rename = "nextPartNumber")]
    pub next_part_number: String,
    /// Always a zero in string form, despite how many parts uploaded.
    #[serde(rename = "maxParts")]
    pub max_parts: String,
}

impl FinishFileDataUploadRequest {
    pub fn new(next_part_number: String, max_parts: String) -> FinishFileDataUploadRequest {
        FinishFileDataUploadRequest {
            etags: None,
            next_part_number,
            max_parts,
        }
    }
}
