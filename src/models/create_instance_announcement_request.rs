use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateInstanceAnnouncementRequest {
    #[serde(rename = "imageId", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "imageVersion", skip_serializing_if = "Option::is_none")]
    pub image_version: Option<i32>,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "title")]
    pub title: String,
}

impl CreateInstanceAnnouncementRequest {
    pub fn new(message: String, title: String) -> CreateInstanceAnnouncementRequest {
        CreateInstanceAnnouncementRequest {
            image_id: None,
            image_version: None,
            message,
            title,
        }
    }
}
