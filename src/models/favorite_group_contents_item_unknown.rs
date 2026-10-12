use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FavoriteGroupContentsItemUnknown {
    #[serde(flatten)]
    pub favorite_group_contents_item_base: models::FavoriteGroupContentsItemBase,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl FavoriteGroupContentsItemUnknown {
    pub fn new(
        favorite_group_contents_item_base: models::FavoriteGroupContentsItemBase,
        r#type: String,
    ) -> FavoriteGroupContentsItemUnknown {
        FavoriteGroupContentsItemUnknown {
            favorite_group_contents_item_base,
            r#type,
        }
    }
}
