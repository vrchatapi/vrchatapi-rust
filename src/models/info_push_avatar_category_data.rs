use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushAvatarCategoryData {
    #[serde(rename = "contentList")]
    pub content_list: models::DynamicContentRow,
    #[serde(rename = "imageUrl", deserialize_with = "Option::deserialize")]
    pub image_url: Option<String>,
}

impl InfoPushAvatarCategoryData {
    pub fn new(
        content_list: models::DynamicContentRow,
        image_url: Option<String>,
    ) -> InfoPushAvatarCategoryData {
        InfoPushAvatarCategoryData {
            content_list,
            image_url,
        }
    }
}
