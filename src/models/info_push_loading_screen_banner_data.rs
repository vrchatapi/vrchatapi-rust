use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushLoadingScreenBannerData {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "imageUrl", deserialize_with = "Option::deserialize")]
    pub image_url: Option<String>,
    #[serde(rename = "name")]
    pub name: models::LocalizableString,
    #[serde(rename = "weight")]
    pub weight: i32,
}

impl InfoPushLoadingScreenBannerData {
    pub fn new(
        category: String,
        image_url: Option<String>,
        name: models::LocalizableString,
        weight: i32,
    ) -> InfoPushLoadingScreenBannerData {
        InfoPushLoadingScreenBannerData {
            category,
            image_url,
            name,
            weight,
        }
    }
}
