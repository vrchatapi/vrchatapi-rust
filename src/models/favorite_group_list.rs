use crate::models;
use serde::{Deserialize, Serialize};

/// FavoriteGroupList : A user's favorite groups of one type, with the limits that apply to them.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FavoriteGroupList {
    #[serde(rename = "favoriteGroups")]
    pub favorite_groups: Vec<models::FavoriteGroupSummary>,
    /// Only returned when the owner is the currently authenticated user.
    #[serde(rename = "maxFavoriteGroups", skip_serializing_if = "Option::is_none")]
    pub max_favorite_groups: Option<i32>,
    /// Only returned when the owner is the currently authenticated user.
    #[serde(
        rename = "maxFavoritesPerGroup",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_favorites_per_group: Option<i32>,
}

impl FavoriteGroupList {
    /// A user's favorite groups of one type, with the limits that apply to them.
    pub fn new(favorite_groups: Vec<models::FavoriteGroupSummary>) -> FavoriteGroupList {
        FavoriteGroupList {
            favorite_groups,
            max_favorite_groups: None,
            max_favorites_per_group: None,
        }
    }
}
