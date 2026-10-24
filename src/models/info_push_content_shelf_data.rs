use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushContentShelfData {
    #[serde(rename = "contentSource")]
    pub content_source: models::InfoPushDataContentSource,
    #[serde(rename = "controls")]
    pub controls: Vec<models::InfoPushDataControl>,
    #[serde(rename = "imageUrl", deserialize_with = "Option::deserialize")]
    pub image_url: Option<String>,
    #[serde(rename = "onPressed", deserialize_with = "Option::deserialize")]
    pub on_pressed: Option<models::InfoPushDataClickable>,
    #[serde(rename = "presentation")]
    pub presentation: models::InfoPushDataPresentation,
    #[serde(rename = "schemaVersion")]
    pub schema_version: i32,
    #[serde(rename = "subtitle")]
    pub subtitle: String,
    #[serde(rename = "thumbnailImageUrl", deserialize_with = "Option::deserialize")]
    pub thumbnail_image_url: Option<String>,
    #[serde(rename = "title")]
    pub title: String,
}

impl InfoPushContentShelfData {
    pub fn new(
        content_source: models::InfoPushDataContentSource,
        controls: Vec<models::InfoPushDataControl>,
        image_url: Option<String>,
        on_pressed: Option<models::InfoPushDataClickable>,
        presentation: models::InfoPushDataPresentation,
        schema_version: i32,
        subtitle: String,
        thumbnail_image_url: Option<String>,
        title: String,
    ) -> InfoPushContentShelfData {
        InfoPushContentShelfData {
            content_source,
            controls,
            image_url,
            on_pressed,
            presentation,
            schema_version,
            subtitle,
            thumbnail_image_url,
            title,
        }
    }
}
