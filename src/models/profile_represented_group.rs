use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileRepresentedGroup {
    #[serde(rename = "bannerUrl", deserialize_with = "Option::deserialize")]
    pub banner_url: Option<String>,
    #[serde(rename = "iconUrl", deserialize_with = "Option::deserialize")]
    pub icon_url: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
}

impl ProfileRepresentedGroup {
    pub fn new(
        banner_url: Option<String>,
        icon_url: Option<String>,
        id: String,
        name: String,
    ) -> ProfileRepresentedGroup {
        ProfileRepresentedGroup {
            banner_url,
            icon_url,
            id,
            name,
        }
    }
}
