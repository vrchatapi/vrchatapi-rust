use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BannerType {
    #[serde(rename = "avatarBanner")]
    AvatarBanner,
    #[serde(rename = "color")]
    Color,
    #[serde(rename = "customImage")]
    CustomImage,
}

impl std::fmt::Display for BannerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AvatarBanner => write!(f, "avatarBanner"),
            Self::Color => write!(f, "color"),
            Self::CustomImage => write!(f, "customImage"),
        }
    }
}

impl Default for BannerType {
    fn default() -> BannerType {
        Self::AvatarBanner
    }
}
