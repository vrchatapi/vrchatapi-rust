use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FavoriteGroupContentsItemWorld {
    #[serde(flatten)]
    pub favorite_group_contents_item_base: models::FavoriteGroupContentsItemBase,
    #[serde(rename = "type", default)]
    pub r#type: Type,
    #[serde(rename = "world", skip_serializing_if = "Option::is_none")]
    pub world: Option<models::FavoriteGroupWorld>,
}

impl FavoriteGroupContentsItemWorld {
    pub fn new(
        favorite_group_contents_item_base: models::FavoriteGroupContentsItemBase,
        r#type: Type,
    ) -> FavoriteGroupContentsItemWorld {
        FavoriteGroupContentsItemWorld {
            favorite_group_contents_item_base,
            r#type,
            world: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "world")]
    World,
}

impl Default for Type {
    fn default() -> Type {
        Self::World
    }
}
