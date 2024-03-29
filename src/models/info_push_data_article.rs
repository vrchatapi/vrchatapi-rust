/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InfoPushDataArticle {
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<Box<crate::models::InfoPushDataArticleContent>>,
}

impl InfoPushDataArticle {
    pub fn new() -> InfoPushDataArticle {
        InfoPushDataArticle {
            content: None,
        }
    }
}


