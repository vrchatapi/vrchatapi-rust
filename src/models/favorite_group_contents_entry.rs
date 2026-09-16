use crate::models;
use serde::{Deserialize, Serialize};

/// FavoriteGroupContentsEntry : A favorite alongside the object it points at. The object appears under a property named for the favorite's type.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FavoriteGroupContentsEntry {
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<models::Avatar>,
    #[serde(rename = "favoriteId")]
    pub favorite_id: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "type")]
    pub r#type: models::FavoriteType,
    #[serde(rename = "world", skip_serializing_if = "Option::is_none")]
    pub world: Option<models::FavoriteGroupContentsEntryWorld>,
}

impl FavoriteGroupContentsEntry {
    /// A favorite alongside the object it points at. The object appears under a property named for the favorite's type.
    pub fn new(
        favorite_id: String,
        id: String,
        tags: Vec<String>,
        r#type: models::FavoriteType,
    ) -> FavoriteGroupContentsEntry {
        FavoriteGroupContentsEntry {
            avatar: None,
            favorite_id,
            id,
            tags,
            r#type,
            world: None,
        }
    }
}
