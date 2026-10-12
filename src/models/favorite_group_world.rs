use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FavoriteGroupWorld {
    FavoritedWorld(models::FavoritedWorld),
    UnavailableWorld(models::UnavailableWorld),
}

impl Default for FavoriteGroupWorld {
    fn default() -> Self {
        Self::FavoritedWorld(Default::default())
    }
}

impl From<models::FavoritedWorld> for FavoriteGroupWorld {
    fn from(value: models::FavoritedWorld) -> Self {
        Self::FavoritedWorld(value)
    }
}

impl TryFrom<FavoriteGroupWorld> for models::FavoritedWorld {
    type Error = FavoriteGroupWorld;

    fn try_from(value: FavoriteGroupWorld) -> Result<Self, Self::Error> {
        match value {
            FavoriteGroupWorld::FavoritedWorld(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::UnavailableWorld> for FavoriteGroupWorld {
    fn from(value: models::UnavailableWorld) -> Self {
        Self::UnavailableWorld(value)
    }
}

impl TryFrom<FavoriteGroupWorld> for models::UnavailableWorld {
    type Error = FavoriteGroupWorld;

    fn try_from(value: FavoriteGroupWorld) -> Result<Self, Self::Error> {
        match value {
            FavoriteGroupWorld::UnavailableWorld(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
