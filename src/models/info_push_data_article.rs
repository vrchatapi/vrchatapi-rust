use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataArticle {
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<models::InfoPushDataArticleContent>>,
    #[serde(rename = "embeddedLinkData", skip_serializing_if = "Option::is_none")]
    pub embedded_link_data: Option<Vec<models::InfoPushEmbeddedLink>>,
    #[serde(rename = "jumpLinks", skip_serializing_if = "Option::is_none")]
    pub jump_links: Option<Vec<String>>,
    #[serde(rename = "moreInfoLinks", skip_serializing_if = "Option::is_none")]
    pub more_info_links: Option<Vec<models::InfoPushEmbeddedLink>>,
    #[serde(rename = "sectionLinks", skip_serializing_if = "Option::is_none")]
    pub section_links: Option<Vec<String>>,
}

impl InfoPushDataArticle {
    pub fn new() -> InfoPushDataArticle {
        InfoPushDataArticle {
            content: None,
            embedded_link_data: None,
            jump_links: None,
            more_info_links: None,
            section_links: None,
        }
    }
}
