use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FavoriteGroupContentsEntryWorld {
    FavoritedWorld(models::FavoritedWorld),
    UnavailableWorld(models::UnavailableWorld),
}

impl Default for FavoriteGroupContentsEntryWorld {
    fn default() -> Self {
        Self::FavoritedWorld(Default::default())
    }
}
