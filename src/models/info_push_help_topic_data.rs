use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushHelpTopicData {
    #[serde(flatten)]
    pub info_push_tile_data: models::InfoPushTileData,
    #[serde(rename = "description")]
    pub description: models::LocalizableString,
    #[serde(
        rename = "imageUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_url: Option<Option<String>>,
    #[serde(rename = "ipsQuery")]
    pub ips_query: models::InfoPushIpsQuery,
    #[serde(rename = "isNew")]
    pub is_new: bool,
    #[serde(
        rename = "thumbnailImageUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub thumbnail_image_url: Option<Option<String>>,
}

impl InfoPushHelpTopicData {
    pub fn new(
        info_push_tile_data: models::InfoPushTileData,
        description: models::LocalizableString,
        ips_query: models::InfoPushIpsQuery,
        is_new: bool,
    ) -> InfoPushHelpTopicData {
        InfoPushHelpTopicData {
            info_push_tile_data,
            description,
            image_url: None,
            ips_query,
            is_new,
            thumbnail_image_url: None,
        }
    }
}
