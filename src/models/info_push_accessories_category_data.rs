use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushAccessoriesCategoryData {
    #[serde(rename = "listingIds")]
    pub listing_ids: Vec<String>,
    #[serde(rename = "name")]
    pub name: models::LocalizableString,
}

impl InfoPushAccessoriesCategoryData {
    pub fn new(
        listing_ids: Vec<String>,
        name: models::LocalizableString,
    ) -> InfoPushAccessoriesCategoryData {
        InfoPushAccessoriesCategoryData { listing_ids, name }
    }
}
