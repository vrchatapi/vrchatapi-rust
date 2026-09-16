use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FavoriteGroupContents {
    #[serde(rename = "favorites")]
    pub favorites: Vec<models::FavoriteGroupContentsEntry>,
    #[serde(rename = "totalCount")]
    pub total_count: i32,
}

impl FavoriteGroupContents {
    pub fn new(
        favorites: Vec<models::FavoriteGroupContentsEntry>,
        total_count: i32,
    ) -> FavoriteGroupContents {
        FavoriteGroupContents {
            favorites,
            total_count,
        }
    }
}
