use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDiscoveryCategoryData {
    #[serde(rename = "contentList")]
    pub content_list: models::DynamicContentRow,
    #[serde(rename = "imageUrl", deserialize_with = "Option::deserialize")]
    pub image_url: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<models::LocalizableString>,
    /// Number of rows to render.
    #[serde(rename = "rows", deserialize_with = "Option::deserialize")]
    pub rows: Option<i32>,
    #[serde(rename = "thumbnailImageUrl", deserialize_with = "Option::deserialize")]
    pub thumbnail_image_url: Option<String>,
}

impl InfoPushDiscoveryCategoryData {
    pub fn new(
        content_list: models::DynamicContentRow,
        image_url: Option<String>,
        rows: Option<i32>,
        thumbnail_image_url: Option<String>,
    ) -> InfoPushDiscoveryCategoryData {
        InfoPushDiscoveryCategoryData {
            content_list,
            image_url,
            name: None,
            rows,
            thumbnail_image_url,
        }
    }
}
