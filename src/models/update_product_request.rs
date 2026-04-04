use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "imageId", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(
        rename = "useForSubscriberList",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_for_subscriber_list: Option<bool>,
}

impl UpdateProductRequest {
    pub fn new() -> UpdateProductRequest {
        UpdateProductRequest {
            description: None,
            display_name: None,
            image_id: None,
            tags: None,
            use_for_subscriber_list: None,
        }
    }
}
