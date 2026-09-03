use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicContentRow {
    #[serde(
        rename = "anyStyle",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub any_style: Option<Option<Vec<String>>>,
    #[serde(
        rename = "anyTag",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub any_tag: Option<Option<Vec<String>>>,
    #[serde(rename = "avatarSpecific", skip_serializing_if = "Option::is_none")]
    pub avatar_specific: Option<bool>,
    #[serde(rename = "bannersTag", skip_serializing_if = "Option::is_none")]
    pub banners_tag: Option<String>,
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    #[serde(rename = "featuredResults", skip_serializing_if = "Option::is_none")]
    pub featured_results: Option<String>,
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(rename = "marketplace", skip_serializing_if = "Option::is_none")]
    pub marketplace: Option<String>,
    #[serde(rename = "maxPrice", skip_serializing_if = "Option::is_none")]
    pub max_price: Option<i32>,
    #[serde(rename = "minOccupants", skip_serializing_if = "Option::is_none")]
    pub min_occupants: Option<i32>,
    #[serde(rename = "minPrice", skip_serializing_if = "Option::is_none")]
    pub min_price: Option<i32>,
    #[serde(
        rename = "minimumInterestCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_interest_count: Option<i32>,
    #[serde(
        rename = "minimumRemainingMinutes",
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_remaining_minutes: Option<i32>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "n", skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(rename = "name")]
    pub name: models::DynamicContentRowName,
    #[serde(rename = "nonFeaturedResults", skip_serializing_if = "Option::is_none")]
    pub non_featured_results: Option<String>,
    #[serde(
        rename = "notag",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub notag: Option<Option<Vec<String>>>,
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(
        rename = "personalizedResults",
        skip_serializing_if = "Option::is_none"
    )]
    pub personalized_results: Option<String>,
    /// This is normally `android`, `ios`, `standalonewindows`, `web`, or the empty value ``, but also supposedly can be any random Unity version such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`.
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(
        rename = "region",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub region: Option<Option<String>>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(
        rename = "shortName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_name: Option<Option<models::DynamicContentRowShortName>>,
    #[serde(rename = "sortHeading", skip_serializing_if = "Option::is_none")]
    pub sort_heading: Option<String>,
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "sortOwnership", skip_serializing_if = "Option::is_none")]
    pub sort_ownership: Option<String>,
    #[serde(
        rename = "style",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub style: Option<Option<String>>,
    /// Tag to filter content for this row. Not a `Tag`: that type forbids the empty string, which this field uses for a row that is not tag-filtered.
    #[serde(
        rename = "tag",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tag: Option<Option<String>>,
    #[serde(
        rename = "tags",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tags: Option<Option<Vec<String>>>,
    /// Type is not present if it is a world.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(
        rename = "upcomingOffsetMinutes",
        skip_serializing_if = "Option::is_none"
    )]
    pub upcoming_offset_minutes: Option<i32>,
}

impl DynamicContentRow {
    pub fn new(name: models::DynamicContentRowName, platform: String) -> DynamicContentRow {
        DynamicContentRow {
            any_style: None,
            any_tag: None,
            avatar_specific: None,
            banners_tag: None,
            categories: None,
            featured_results: None,
            index: None,
            marketplace: None,
            max_price: None,
            min_occupants: None,
            min_price: None,
            minimum_interest_count: None,
            minimum_remaining_minutes: None,
            mode: None,
            n: None,
            name,
            non_featured_results: None,
            notag: None,
            params: None,
            personalized_results: None,
            platform,
            region: None,
            scope: None,
            short_name: None,
            sort_heading: None,
            sort_order: None,
            sort_ownership: None,
            style: None,
            tag: None,
            tags: None,
            r#type: None,
            upcoming_offset_minutes: None,
        }
    }
}
