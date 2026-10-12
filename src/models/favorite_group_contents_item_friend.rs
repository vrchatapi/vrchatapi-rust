use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FavoriteGroupContentsItemFriend {
    #[serde(flatten)]
    pub favorite_group_contents_item_base: models::FavoriteGroupContentsItemBase,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl FavoriteGroupContentsItemFriend {
    pub fn new(
        favorite_group_contents_item_base: models::FavoriteGroupContentsItemBase,
        r#type: Type,
    ) -> FavoriteGroupContentsItemFriend {
        FavoriteGroupContentsItemFriend {
            favorite_group_contents_item_base,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "friend")]
    Friend,
}

impl Default for Type {
    fn default() -> Type {
        Self::Friend
    }
}
