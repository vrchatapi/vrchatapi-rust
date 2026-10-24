use crate::models;
use serde::{Deserialize, Serialize};

/// The data of an item without `type`. Its fields depend on the item's tags.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InfoPushData {
    InfoPushOnboardingAvatarData(models::InfoPushOnboardingAvatarData),
    InfoPushHelpTopicData(models::InfoPushHelpTopicData),
    InfoPushDiscoveryCategoryData(models::InfoPushDiscoveryCategoryData),
    InfoPushHelpArticleData(models::InfoPushHelpArticleData),
    InfoPushLoadingScreenBannerData(models::InfoPushLoadingScreenBannerData),
    InfoPushMarketplacePageData(models::InfoPushMarketplacePageData),
    InfoPushWorldCategoryData(models::InfoPushWorldCategoryData),
    InfoPushUrlConsentScreenData(models::InfoPushUrlConsentScreenData),
    InfoPushAvatarCategoryData(models::InfoPushAvatarCategoryData),
    InfoPushSearchData(models::InfoPushSearchData),
    InfoPushAccessoriesCategoryData(models::InfoPushAccessoriesCategoryData),
    InfoPushBannerData(models::InfoPushBannerData),
    InfoPushVersionData(models::InfoPushVersionData),
    InfoPushOtherData(models::InfoPushOtherData),
}

impl Default for InfoPushData {
    fn default() -> Self {
        Self::InfoPushOnboardingAvatarData(Default::default())
    }
}

impl From<models::InfoPushOnboardingAvatarData> for InfoPushData {
    fn from(value: models::InfoPushOnboardingAvatarData) -> Self {
        Self::InfoPushOnboardingAvatarData(value)
    }
}

impl TryFrom<InfoPushData> for models::InfoPushOnboardingAvatarData {
    type Error = InfoPushData;

    fn try_from(value: InfoPushData) -> Result<Self, Self::Error> {
        match value {
            InfoPushData::InfoPushOnboardingAvatarData(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushHelpTopicData> for InfoPushData {
    fn from(value: models::InfoPushHelpTopicData) -> Self {
        Self::InfoPushHelpTopicData(value)
    }
}

impl TryFrom<InfoPushData> for models::InfoPushHelpTopicData {
    type Error = InfoPushData;

    fn try_from(value: InfoPushData) -> Result<Self, Self::Error> {
        match value {
            InfoPushData::InfoPushHelpTopicData(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushDiscoveryCategoryData> for InfoPushData {
    fn from(value: models::InfoPushDiscoveryCategoryData) -> Self {
        Self::InfoPushDiscoveryCategoryData(value)
    }
}

impl TryFrom<InfoPushData> for models::InfoPushDiscoveryCategoryData {
    type Error = InfoPushData;

    fn try_from(value: InfoPushData) -> Result<Self, Self::Error> {
        match value {
            InfoPushData::InfoPushDiscoveryCategoryData(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushHelpArticleData> for InfoPushData {
    fn from(value: models::InfoPushHelpArticleData) -> Self {
        Self::InfoPushHelpArticleData(value)
    }
}

impl TryFrom<InfoPushData> for models::InfoPushHelpArticleData {
    type Error = InfoPushData;

    fn try_from(value: InfoPushData) -> Result<Self, Self::Error> {
        match value {
            InfoPushData::InfoPushHelpArticleData(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushLoadingScreenBannerData> for InfoPushData {
    fn from(value: models::InfoPushLoadingScreenBannerData) -> Self {
        Self::InfoPushLoadingScreenBannerData(value)
    }
}

impl TryFrom<InfoPushData> for models::InfoPushLoadingScreenBannerData {
    type Error = InfoPushData;

    fn try_from(value: InfoPushData) -> Result<Self, Self::Error> {
        match value {
            InfoPushData::InfoPushLoadingScreenBannerData(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushMarketplacePageData> for InfoPushData {
    fn from(value: models::InfoPushMarketplacePageData) -> Self {
        Self::InfoPushMarketplacePageData(value)
    }
}

impl TryFrom<InfoPushData> for models::InfoPushMarketplacePageData {
    type Error = InfoPushData;

    fn try_from(value: InfoPushData) -> Result<Self, Self::Error> {
        match value {
            InfoPushData::InfoPushMarketplacePageData(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushWorldCategoryData> for InfoPushData {
    fn from(value: models::InfoPushWorldCategoryData) -> Self {
        Self::InfoPushWorldCategoryData(value)
    }
}

impl TryFrom<InfoPushData> for models::InfoPushWorldCategoryData {
    type Error = InfoPushData;

    fn try_from(value: InfoPushData) -> Result<Self, Self::Error> {
        match value {
            InfoPushData::InfoPushWorldCategoryData(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushUrlConsentScreenData> for InfoPushData {
    fn from(value: models::InfoPushUrlConsentScreenData) -> Self {
        Self::InfoPushUrlConsentScreenData(value)
    }
}

impl TryFrom<InfoPushData> for models::InfoPushUrlConsentScreenData {
    type Error = InfoPushData;

    fn try_from(value: InfoPushData) -> Result<Self, Self::Error> {
        match value {
            InfoPushData::InfoPushUrlConsentScreenData(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushAvatarCategoryData> for InfoPushData {
    fn from(value: models::InfoPushAvatarCategoryData) -> Self {
        Self::InfoPushAvatarCategoryData(value)
    }
}

impl TryFrom<InfoPushData> for models::InfoPushAvatarCategoryData {
    type Error = InfoPushData;

    fn try_from(value: InfoPushData) -> Result<Self, Self::Error> {
        match value {
            InfoPushData::InfoPushAvatarCategoryData(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushSearchData> for InfoPushData {
    fn from(value: models::InfoPushSearchData) -> Self {
        Self::InfoPushSearchData(value)
    }
}

impl TryFrom<InfoPushData> for models::InfoPushSearchData {
    type Error = InfoPushData;

    fn try_from(value: InfoPushData) -> Result<Self, Self::Error> {
        match value {
            InfoPushData::InfoPushSearchData(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushAccessoriesCategoryData> for InfoPushData {
    fn from(value: models::InfoPushAccessoriesCategoryData) -> Self {
        Self::InfoPushAccessoriesCategoryData(value)
    }
}

impl TryFrom<InfoPushData> for models::InfoPushAccessoriesCategoryData {
    type Error = InfoPushData;

    fn try_from(value: InfoPushData) -> Result<Self, Self::Error> {
        match value {
            InfoPushData::InfoPushAccessoriesCategoryData(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushBannerData> for InfoPushData {
    fn from(value: models::InfoPushBannerData) -> Self {
        Self::InfoPushBannerData(value)
    }
}

impl TryFrom<InfoPushData> for models::InfoPushBannerData {
    type Error = InfoPushData;

    fn try_from(value: InfoPushData) -> Result<Self, Self::Error> {
        match value {
            InfoPushData::InfoPushBannerData(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushVersionData> for InfoPushData {
    fn from(value: models::InfoPushVersionData) -> Self {
        Self::InfoPushVersionData(value)
    }
}

impl TryFrom<InfoPushData> for models::InfoPushVersionData {
    type Error = InfoPushData;

    fn try_from(value: InfoPushData) -> Result<Self, Self::Error> {
        match value {
            InfoPushData::InfoPushVersionData(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushOtherData> for InfoPushData {
    fn from(value: models::InfoPushOtherData) -> Self {
        Self::InfoPushOtherData(value)
    }
}

impl TryFrom<InfoPushData> for models::InfoPushOtherData {
    type Error = InfoPushData;

    fn try_from(value: InfoPushData) -> Result<Self, Self::Error> {
        match value {
            InfoPushData::InfoPushOtherData(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
