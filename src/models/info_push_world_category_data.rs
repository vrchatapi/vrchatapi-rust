use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushWorldCategoryData {
    #[serde(flatten)]
    pub info_push_tile_data: models::InfoPushTileData,
    #[serde(rename = "contentList")]
    pub content_list: models::DynamicContentRow,
    #[serde(rename = "hoverToJoin", skip_serializing_if = "Option::is_none")]
    pub hover_to_join: Option<bool>,
    #[serde(
        rename = "shortName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_name: Option<Option<models::LocalizableString>>,
    #[serde(
        rename = "thumbnailImageUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub thumbnail_image_url: Option<Option<String>>,
}

impl InfoPushWorldCategoryData {
    pub fn new(
        info_push_tile_data: models::InfoPushTileData,
        content_list: models::DynamicContentRow,
    ) -> InfoPushWorldCategoryData {
        InfoPushWorldCategoryData {
            info_push_tile_data,
            content_list,
            hover_to_join: None,
            short_name: None,
            thumbnail_image_url: None,
        }
    }
}
