use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    #[serde(
        rename = "backgroundTextureId",
        skip_serializing_if = "Option::is_none"
    )]
    pub background_texture_id: Option<String>,
    #[serde(rename = "backgroundType", skip_serializing_if = "Option::is_none")]
    pub background_type: Option<BackgroundType>,
    /// Hex colour without a leading `#`.
    #[serde(rename = "bannerColor", skip_serializing_if = "Option::is_none")]
    pub banner_color: Option<String>,
    #[serde(rename = "bannerType", skip_serializing_if = "Option::is_none")]
    pub banner_type: Option<BannerType>,
    #[serde(rename = "bio", skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(rename = "bioLinks", skip_serializing_if = "Option::is_none")]
    pub bio_links: Option<Vec<String>>,
    #[serde(rename = "iconFrame", skip_serializing_if = "Option::is_none")]
    pub icon_frame: Option<String>,
    #[serde(rename = "languages", skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(rename = "nameplateEffect", skip_serializing_if = "Option::is_none")]
    pub nameplate_effect: Option<String>,
    #[serde(rename = "profileEffect", skip_serializing_if = "Option::is_none")]
    pub profile_effect: Option<String>,
    #[serde(rename = "themeId", skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
    #[serde(rename = "userIcon", skip_serializing_if = "Option::is_none")]
    pub user_icon: Option<String>,
}

impl UpdateProfileRequest {
    pub fn new() -> UpdateProfileRequest {
        UpdateProfileRequest {
            background_texture_id: None,
            background_type: None,
            banner_color: None,
            banner_type: None,
            bio: None,
            bio_links: None,
            icon_frame: None,
            languages: None,
            nameplate_effect: None,
            profile_effect: None,
            theme_id: None,
            user_icon: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BackgroundType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "gradient")]
    Gradient,
    #[serde(rename = "inventory")]
    Inventory,
    #[serde(rename = "texture")]
    Texture,
}

impl Default for BackgroundType {
    fn default() -> BackgroundType {
        Self::Default
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BannerType {
    #[serde(rename = "avatarBanner")]
    AvatarBanner,
    #[serde(rename = "color")]
    Color,
    #[serde(rename = "customImage")]
    CustomImage,
}

impl Default for BannerType {
    fn default() -> BannerType {
        Self::AvatarBanner
    }
}
