use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushHelpArticleData {
    #[serde(rename = "article")]
    pub article: models::InfoPushDataArticle,
    #[serde(rename = "description")]
    pub description: models::LocalizableString,
    #[serde(rename = "imageUrl", deserialize_with = "Option::deserialize")]
    pub image_url: Option<String>,
    #[serde(rename = "name")]
    pub name: models::LocalizableString,
}

impl InfoPushHelpArticleData {
    pub fn new(
        article: models::InfoPushDataArticle,
        description: models::LocalizableString,
        image_url: Option<String>,
        name: models::LocalizableString,
    ) -> InfoPushHelpArticleData {
        InfoPushHelpArticleData {
            article,
            description,
            image_url,
            name,
        }
    }
}
