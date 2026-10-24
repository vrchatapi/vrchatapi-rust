use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicContentRowAvatars {
    #[serde(flatten)]
    pub dynamic_content_row_base: models::DynamicContentRowBase,
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
    #[serde(rename = "marketplace", skip_serializing_if = "Option::is_none")]
    pub marketplace: Option<String>,
    #[serde(rename = "maxPrice", skip_serializing_if = "Option::is_none")]
    pub max_price: Option<i32>,
    #[serde(rename = "minPrice", skip_serializing_if = "Option::is_none")]
    pub min_price: Option<i32>,
    #[serde(
        rename = "notag",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub notag: Option<Option<Vec<String>>>,
    #[serde(rename = "params")]
    pub params: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "sortHeading")]
    pub sort_heading: String,
    #[serde(rename = "sortOrder")]
    pub sort_order: models::OrderOption,
    #[serde(rename = "sortOwnership")]
    pub sort_ownership: String,
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
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl DynamicContentRowAvatars {
    pub fn new(
        dynamic_content_row_base: models::DynamicContentRowBase,
        params: std::collections::HashMap<String, serde_json::Value>,
        sort_heading: String,
        sort_order: models::OrderOption,
        sort_ownership: String,
        r#type: Type,
    ) -> DynamicContentRowAvatars {
        DynamicContentRowAvatars {
            dynamic_content_row_base,
            any_style: None,
            any_tag: None,
            marketplace: None,
            max_price: None,
            min_price: None,
            notag: None,
            params,
            sort_heading,
            sort_order,
            sort_ownership,
            style: None,
            tag: None,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "avatar")]
    Avatar,
}

impl Default for Type {
    fn default() -> Type {
        Self::Avatar
    }
}
