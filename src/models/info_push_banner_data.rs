use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushBannerData {
    #[serde(rename = "imageUrl", deserialize_with = "Option::deserialize")]
    pub image_url: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<models::LocalizableString>,
    #[serde(
        rename = "onPressed",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub on_pressed: Option<Option<models::InfoPushDataClickable>>,
}

impl InfoPushBannerData {
    pub fn new(image_url: Option<String>) -> InfoPushBannerData {
        InfoPushBannerData {
            image_url,
            name: None,
            on_pressed: None,
        }
    }
}
