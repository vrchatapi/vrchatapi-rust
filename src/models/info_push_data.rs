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
    #[serde(rename = "contentSource", skip_serializing_if = "Option::is_none")]
    pub content_source: Option<models::InfoPushDataContentSource>,
    #[serde(rename = "controls", skip_serializing_if = "Option::is_none")]
    pub controls: Option<Vec<models::InfoPushDataControl>>,
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
    #[serde(
        rename = "onPressed",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub on_pressed: Option<Option<models::InfoPushDataClickable>>,
    /// An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance.
    #[serde(
        rename = "overrideName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub override_name: Option<Option<serde_json::Value>>,
    #[serde(rename = "presentation", skip_serializing_if = "Option::is_none")]
    pub presentation: Option<models::InfoPushDataPresentation>,
    #[serde(rename = "promotion", skip_serializing_if = "Option::is_none")]
    pub promotion: Option<models::InfoPushDataPromotion>,
    /// Number of rows to render.
    #[serde(
        rename = "rows",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub rows: Option<Option<i32>>,
    #[serde(rename = "schemaVersion", skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<i32>,
    #[serde(rename = "search", skip_serializing_if = "Option::is_none")]
    pub search: Option<models::InfoPushDataSearch>,
    #[serde(
        rename = "shortName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_name: Option<Option<models::DynamicContentRowShortName>>,
    /// An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance.
    #[serde(
        rename = "showInWorldIds",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_in_world_ids: Option<Option<serde_json::Value>>,
    #[serde(rename = "subtitle", skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
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
            content_source: None,
            controls: None,
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
            presentation: None,
            promotion: None,
            rows: None,
            schema_version: None,
            search: None,
            short_name: None,
            show_in_world_ids: None,
            subtitle: None,
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
