use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FavoriteGroupContentsItemAvatar {
    #[serde(flatten)]
    pub favorite_group_contents_item_base: models::FavoriteGroupContentsItemBase,
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<models::Avatar>,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl FavoriteGroupContentsItemAvatar {
    pub fn new(
        favorite_group_contents_item_base: models::FavoriteGroupContentsItemBase,
        r#type: Type,
    ) -> FavoriteGroupContentsItemAvatar {
        FavoriteGroupContentsItemAvatar {
            favorite_group_contents_item_base,
            avatar: None,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "avatar")]
    Avatar,
}

impl Default for Type {
    fn default() -> Type {
        Self::Avatar
    }
}
