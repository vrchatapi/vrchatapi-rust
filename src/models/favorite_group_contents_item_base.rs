use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FavoriteGroupContentsItemBase {
    #[serde(rename = "favoriteId")]
    pub favorite_id: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

impl FavoriteGroupContentsItemBase {
    pub fn new(
        favorite_id: String,
        id: String,
        tags: Vec<String>,
    ) -> FavoriteGroupContentsItemBase {
        FavoriteGroupContentsItemBase {
            favorite_id,
            id,
            tags,
        }
    }
}
