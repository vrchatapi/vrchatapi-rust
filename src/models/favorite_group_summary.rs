use crate::models;
use serde::{Deserialize, Serialize};

/// FavoriteGroupSummary : A favorite group as listed for one favorite type.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FavoriteGroupSummary {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "numFavorites")]
    pub num_favorites: i32,
    #[serde(rename = "visibility")]
    pub visibility: models::FavoriteGroupVisibility,
}

impl FavoriteGroupSummary {
    /// A favorite group as listed for one favorite type.
    pub fn new(
        display_name: String,
        id: String,
        name: String,
        num_favorites: i32,
        visibility: models::FavoriteGroupVisibility,
    ) -> FavoriteGroupSummary {
        FavoriteGroupSummary {
            display_name,
            id,
            name,
            num_favorites,
            visibility,
        }
    }
}
