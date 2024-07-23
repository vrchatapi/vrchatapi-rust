/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicContentRow {
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
    /// Usually \"ThisPlatformSupported\", but can also be other values such as \"all\" or platform specific identifiers.
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "sortHeading")]
    pub sort_heading: String,
    #[serde(rename = "sortOrder")]
    pub sort_order: String,
    #[serde(rename = "sortOwnership")]
    pub sort_ownership: String,
    /// Tag to filter content for this row.
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Type is not present if it is a world.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl DynamicContentRow {
    pub fn new(
        name: String,
        platform: String,
        sort_heading: String,
        sort_order: String,
        sort_ownership: String,
    ) -> DynamicContentRow {
        DynamicContentRow {
            index: None,
            name,
            platform,
            sort_heading,
            sort_order,
            sort_ownership,
            tag: None,
            r#type: None,
        }
    }
}
