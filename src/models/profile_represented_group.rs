use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileRepresentedGroup {
    #[serde(
        rename = "bannerUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub banner_url: Option<Option<String>>,
    #[serde(
        rename = "iconUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_url: Option<Option<String>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ProfileRepresentedGroup {
    pub fn new() -> ProfileRepresentedGroup {
        ProfileRepresentedGroup {
            banner_url: None,
            icon_url: None,
            id: None,
            name: None,
        }
    }
}
