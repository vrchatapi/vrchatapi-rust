use crate::models;
use serde::{Deserialize, Serialize};

/// A favorite alongside the object it points at. The object appears under a property named for the favorite's type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FavoriteGroupContentsItem {
    #[serde(rename = "avatar")]
    Avatar(models::FavoriteGroupContentsItemAvatar),
    #[serde(rename = "friend")]
    Friend(models::FavoriteGroupContentsItemFriend),
    #[serde(rename = "vrcPlusWorld")]
    VrcPlusWorld(models::FavoriteGroupContentsItemVrcPlusWorld),
    #[serde(rename = "world")]
    World(models::FavoriteGroupContentsItemWorld),
    #[serde(untagged)]
    FavoriteGroupContentsItemUnknown(models::FavoriteGroupContentsItemUnknown),
}

impl Default for FavoriteGroupContentsItem {
    fn default() -> Self {
        Self::Avatar(Default::default())
    }
}

impl From<models::FavoriteGroupContentsItemAvatar> for FavoriteGroupContentsItem {
    fn from(value: models::FavoriteGroupContentsItemAvatar) -> Self {
        Self::Avatar(value)
    }
}

impl TryFrom<FavoriteGroupContentsItem> for models::FavoriteGroupContentsItemAvatar {
    type Error = FavoriteGroupContentsItem;

    fn try_from(value: FavoriteGroupContentsItem) -> Result<Self, Self::Error> {
        match value {
            FavoriteGroupContentsItem::Avatar(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::FavoriteGroupContentsItemFriend> for FavoriteGroupContentsItem {
    fn from(value: models::FavoriteGroupContentsItemFriend) -> Self {
        Self::Friend(value)
    }
}

impl TryFrom<FavoriteGroupContentsItem> for models::FavoriteGroupContentsItemFriend {
    type Error = FavoriteGroupContentsItem;

    fn try_from(value: FavoriteGroupContentsItem) -> Result<Self, Self::Error> {
        match value {
            FavoriteGroupContentsItem::Friend(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::FavoriteGroupContentsItemVrcPlusWorld> for FavoriteGroupContentsItem {
    fn from(value: models::FavoriteGroupContentsItemVrcPlusWorld) -> Self {
        Self::VrcPlusWorld(value)
    }
}

impl TryFrom<FavoriteGroupContentsItem> for models::FavoriteGroupContentsItemVrcPlusWorld {
    type Error = FavoriteGroupContentsItem;

    fn try_from(value: FavoriteGroupContentsItem) -> Result<Self, Self::Error> {
        match value {
            FavoriteGroupContentsItem::VrcPlusWorld(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::FavoriteGroupContentsItemWorld> for FavoriteGroupContentsItem {
    fn from(value: models::FavoriteGroupContentsItemWorld) -> Self {
        Self::World(value)
    }
}

impl TryFrom<FavoriteGroupContentsItem> for models::FavoriteGroupContentsItemWorld {
    type Error = FavoriteGroupContentsItem;

    fn try_from(value: FavoriteGroupContentsItem) -> Result<Self, Self::Error> {
        match value {
            FavoriteGroupContentsItem::World(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::FavoriteGroupContentsItemUnknown> for FavoriteGroupContentsItem {
    fn from(value: models::FavoriteGroupContentsItemUnknown) -> Self {
        Self::FavoriteGroupContentsItemUnknown(value)
    }
}

impl TryFrom<FavoriteGroupContentsItem> for models::FavoriteGroupContentsItemUnknown {
    type Error = FavoriteGroupContentsItem;

    fn try_from(value: FavoriteGroupContentsItem) -> Result<Self, Self::Error> {
        match value {
            FavoriteGroupContentsItem::FavoriteGroupContentsItemUnknown(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
