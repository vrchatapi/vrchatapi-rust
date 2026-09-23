use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushData {
    #[serde(rename = "article", skip_serializing_if = "Option::is_none")]
    pub article: Option<models::InfoPushDataArticle>,
    #[serde(rename = "authorName", skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    #[serde(rename = "avatarId", skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<String>,
    #[serde(rename = "bannerImageUrl", skip_serializing_if = "Option::is_none")]
    pub banner_image_url: Option<String>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<models::InfoPushDataCategory>>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "contentList", skip_serializing_if = "Option::is_none")]
    pub content_list: Option<models::DynamicContentRow>,
    #[serde(rename = "cta", skip_serializing_if = "Option::is_none")]
    pub cta: Option<models::InfoPushDataCallToAction>,
    #[serde(rename = "deliveryBehavior", skip_serializing_if = "Option::is_none")]
    pub delivery_behavior: Option<models::InfoPushDataDeliveryBehavior>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<models::InfoPushDataCategoryName>,
    #[serde(rename = "disclaimerText", skip_serializing_if = "Option::is_none")]
    pub disclaimer_text: Option<String>,
    #[serde(rename = "domainList", skip_serializing_if = "Option::is_none")]
    pub domain_list: Option<Vec<models::InfoPushDataDomainListInner>>,
    #[serde(
        rename = "featuredAvatarCategoryId",
        skip_serializing_if = "Option::is_none"
    )]
    pub featured_avatar_category_id: Option<String>,
    #[serde(rename = "finalName", skip_serializing_if = "Option::is_none")]
    pub final_name: Option<String>,
    #[serde(rename = "hoverToJoin", skip_serializing_if = "Option::is_none")]
    pub hover_to_join: Option<bool>,
    #[serde(rename = "iconImageUrl", skip_serializing_if = "Option::is_none")]
    pub icon_image_url: Option<String>,
    #[serde(rename = "imageFileId", skip_serializing_if = "Option::is_none")]
    pub image_file_id: Option<String>,
    #[serde(
        rename = "imageUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_url: Option<Option<String>>,
    #[serde(rename = "ipsQuery", skip_serializing_if = "Option::is_none")]
    pub ips_query: Option<models::InfoPushIpsQuery>,
    #[serde(rename = "isNew", skip_serializing_if = "Option::is_none")]
    pub is_new: Option<bool>,
    #[serde(rename = "listingIds", skip_serializing_if = "Option::is_none")]
    pub listing_ids: Option<Vec<String>>,
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<models::InfoPushDataCategoryName>,
    #[serde(rename = "onPressed", skip_serializing_if = "Option::is_none")]
    pub on_pressed: Option<models::InfoPushDataClickable>,
    #[serde(
        rename = "overrideName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub override_name: Option<Option<serde_json::Value>>,
    /// Number of rows to render.
    #[serde(
        rename = "rows",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub rows: Option<Option<i32>>,
    #[serde(rename = "search", skip_serializing_if = "Option::is_none")]
    pub search: Option<models::InfoPushDataSearch>,
    #[serde(
        rename = "shortName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_name: Option<Option<models::DynamicContentRowShortName>>,
    #[serde(
        rename = "showInWorldIds",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_in_world_ids: Option<Option<serde_json::Value>>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(
        rename = "thumbnailImageUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub thumbnail_image_url: Option<Option<String>>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "tooltipDescription", skip_serializing_if = "Option::is_none")]
    pub tooltip_description: Option<models::InfoPushDataCategoryName>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "videoFileId", skip_serializing_if = "Option::is_none")]
    pub video_file_id: Option<String>,
    #[serde(rename = "videoUrl", skip_serializing_if = "Option::is_none")]
    pub video_url: Option<String>,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[serde(rename = "worldTag", skip_serializing_if = "Option::is_none")]
    pub world_tag: Option<String>,
}

impl InfoPushData {
    pub fn new() -> InfoPushData {
        InfoPushData {
            article: None,
            author_name: None,
            avatar_id: None,
            banner_image_url: None,
            body: None,
            categories: None,
            category: None,
            content_list: None,
            cta: None,
            delivery_behavior: None,
            description: None,
            disclaimer_text: None,
            domain_list: None,
            featured_avatar_category_id: None,
            final_name: None,
            hover_to_join: None,
            icon_image_url: None,
            image_file_id: None,
            image_url: None,
            ips_query: None,
            is_new: None,
            listing_ids: None,
            media_type: None,
            name: None,
            on_pressed: None,
            override_name: None,
            rows: None,
            search: None,
            short_name: None,
            show_in_world_ids: None,
            template: None,
            thumbnail_image_url: None,
            title: None,
            tooltip_description: None,
            version: None,
            video_file_id: None,
            video_url: None,
            weight: None,
            world_tag: None,
        }
    }
}
