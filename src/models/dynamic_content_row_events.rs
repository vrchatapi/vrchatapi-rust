use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicContentRowEvents {
    #[serde(flatten)]
    pub dynamic_content_row_base: models::DynamicContentRowBase,
    #[serde(rename = "categories")]
    pub categories: Vec<String>,
    #[serde(rename = "featuredResults")]
    pub featured_results: String,
    #[serde(rename = "minimumInterestCount")]
    pub minimum_interest_count: i32,
    #[serde(rename = "minimumRemainingMinutes")]
    pub minimum_remaining_minutes: i32,
    #[serde(rename = "nonFeaturedResults")]
    pub non_featured_results: String,
    #[serde(rename = "params")]
    pub params: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "personalizedResults")]
    pub personalized_results: String,
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(
        rename = "tags",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tags: Option<Option<Vec<String>>>,
    #[serde(rename = "type", default)]
    pub r#type: Type,
    #[serde(rename = "upcomingOffsetMinutes")]
    pub upcoming_offset_minutes: i32,
}

impl DynamicContentRowEvents {
    pub fn new(
        dynamic_content_row_base: models::DynamicContentRowBase,
        categories: Vec<String>,
        featured_results: String,
        minimum_interest_count: i32,
        minimum_remaining_minutes: i32,
        non_featured_results: String,
        params: std::collections::HashMap<String, serde_json::Value>,
        personalized_results: String,
        scope: String,
        r#type: Type,
        upcoming_offset_minutes: i32,
    ) -> DynamicContentRowEvents {
        DynamicContentRowEvents {
            dynamic_content_row_base,
            categories,
            featured_results,
            minimum_interest_count,
            minimum_remaining_minutes,
            non_featured_results,
            params,
            personalized_results,
            scope,
            tags: None,
            r#type,
            upcoming_offset_minutes,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "events")]
    Events,
}

impl Default for Type {
    fn default() -> Type {
        Self::Events
    }
}
