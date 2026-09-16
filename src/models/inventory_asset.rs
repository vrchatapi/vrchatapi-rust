use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InventoryAsset {
    #[serde(rename = "fileId", skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(rename = "frameCount", skip_serializing_if = "Option::is_none")]
    pub frame_count: Option<i32>,
    #[serde(rename = "framesPerSecond", skip_serializing_if = "Option::is_none")]
    pub frames_per_second: Option<f64>,
    #[serde(rename = "loopCount", skip_serializing_if = "Option::is_none")]
    pub loop_count: Option<i32>,
    #[serde(rename = "totalDurationMs", skip_serializing_if = "Option::is_none")]
    pub total_duration_ms: Option<i32>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "url")]
    pub url: String,
}

impl InventoryAsset {
    pub fn new(r#type: String, url: String) -> InventoryAsset {
        InventoryAsset {
            file_id: None,
            frame_count: None,
            frames_per_second: None,
            loop_count: None,
            total_duration_ms: None,
            r#type,
            url,
        }
    }
}
