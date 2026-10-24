use crate::models;
use serde::{Deserialize, Serialize};

/// A category of content. The fields depend on `type`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InfoPushDataCategory {
    #[serde(rename = "articles")]
    Articles(models::InfoPushDataCategoryArticles),
    #[serde(rename = "groups")]
    Groups(models::InfoPushDataCategoryGroups),
    #[serde(rename = "worlds")]
    Worlds(models::InfoPushDataCategoryWorlds),
    #[serde(untagged)]
    InfoPushDataCategoryUnknown(models::InfoPushDataCategoryUnknown),
}

impl Default for InfoPushDataCategory {
    fn default() -> Self {
        Self::Articles(Default::default())
    }
}

impl From<models::InfoPushDataCategoryArticles> for InfoPushDataCategory {
    fn from(value: models::InfoPushDataCategoryArticles) -> Self {
        Self::Articles(value)
    }
}

impl TryFrom<InfoPushDataCategory> for models::InfoPushDataCategoryArticles {
    type Error = InfoPushDataCategory;

    fn try_from(value: InfoPushDataCategory) -> Result<Self, Self::Error> {
        match value {
            InfoPushDataCategory::Articles(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushDataCategoryGroups> for InfoPushDataCategory {
    fn from(value: models::InfoPushDataCategoryGroups) -> Self {
        Self::Groups(value)
    }
}

impl TryFrom<InfoPushDataCategory> for models::InfoPushDataCategoryGroups {
    type Error = InfoPushDataCategory;

    fn try_from(value: InfoPushDataCategory) -> Result<Self, Self::Error> {
        match value {
            InfoPushDataCategory::Groups(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushDataCategoryWorlds> for InfoPushDataCategory {
    fn from(value: models::InfoPushDataCategoryWorlds) -> Self {
        Self::Worlds(value)
    }
}

impl TryFrom<InfoPushDataCategory> for models::InfoPushDataCategoryWorlds {
    type Error = InfoPushDataCategory;

    fn try_from(value: InfoPushDataCategory) -> Result<Self, Self::Error> {
        match value {
            InfoPushDataCategory::Worlds(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushDataCategoryUnknown> for InfoPushDataCategory {
    fn from(value: models::InfoPushDataCategoryUnknown) -> Self {
        Self::InfoPushDataCategoryUnknown(value)
    }
}

impl TryFrom<InfoPushDataCategory> for models::InfoPushDataCategoryUnknown {
    type Error = InfoPushDataCategory;

    fn try_from(value: InfoPushDataCategory) -> Result<Self, Self::Error> {
        match value {
            InfoPushDataCategory::InfoPushDataCategoryUnknown(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
