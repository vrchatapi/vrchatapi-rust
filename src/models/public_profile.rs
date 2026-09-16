use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicProfile {
    #[serde(
        rename = "ageVerificationStatus",
        skip_serializing_if = "Option::is_none"
    )]
    pub age_verification_status: Option<models::AgeVerificationStatus>,
    /// `true` if, user is age verified (not 18+).
    #[serde(rename = "ageVerified", skip_serializing_if = "Option::is_none")]
    pub age_verified: Option<bool>,
    #[serde(
        rename = "backgroundTextureId",
        skip_serializing_if = "Option::is_none"
    )]
    pub background_texture_id: Option<String>,
    #[serde(rename = "backgroundType", skip_serializing_if = "Option::is_none")]
    pub background_type: Option<String>,
    #[serde(rename = "badges", skip_serializing_if = "Option::is_none")]
    pub badges: Option<Vec<models::Badge>>,
    #[serde(rename = "bannerColor", skip_serializing_if = "Option::is_none")]
    pub banner_color: Option<String>,
    #[serde(rename = "bannerType", skip_serializing_if = "Option::is_none")]
    pub banner_type: Option<String>,
    #[serde(rename = "bannerUrl", skip_serializing_if = "Option::is_none")]
    pub banner_url: Option<String>,
    #[serde(rename = "bio", skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(rename = "bioLinks", skip_serializing_if = "Option::is_none")]
    pub bio_links: Option<Vec<String>>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<models::ProfileGroups>,
    #[serde(rename = "hasVrcPlus", skip_serializing_if = "Option::is_none")]
    pub has_vrc_plus: Option<bool>,
    #[serde(rename = "iconFrame", skip_serializing_if = "Option::is_none")]
    pub icon_frame: Option<String>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isEconomyCreator", skip_serializing_if = "Option::is_none")]
    pub is_economy_creator: Option<bool>,
    #[serde(rename = "languages", skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(rename = "nameplateEffect", skip_serializing_if = "Option::is_none")]
    pub nameplate_effect: Option<String>,
    #[serde(rename = "profileEffect", skip_serializing_if = "Option::is_none")]
    pub profile_effect: Option<String>,
    #[serde(rename = "pronouns", skip_serializing_if = "Option::is_none")]
    pub pronouns: Option<String>,
    #[serde(rename = "publicWorlds", skip_serializing_if = "Option::is_none")]
    pub public_worlds: Option<Vec<models::LimitedWorld>>,
    #[serde(rename = "representedGroup", skip_serializing_if = "Option::is_none")]
    pub represented_group: Option<models::ProfileRepresentedGroup>,
    /// Hex colour without a leading `#`.
    #[serde(rename = "themeButtonColor", skip_serializing_if = "Option::is_none")]
    pub theme_button_color: Option<String>,
    /// Hex colour without a leading `#`.
    #[serde(rename = "themeIconColor", skip_serializing_if = "Option::is_none")]
    pub theme_icon_color: Option<String>,
    #[serde(rename = "themeId", skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
    /// Hex colour without a leading `#`.
    #[serde(rename = "themeSubtextColor", skip_serializing_if = "Option::is_none")]
    pub theme_subtext_color: Option<String>,
    #[serde(
        rename = "totalPublicWorldsCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_public_worlds_count: Option<i32>,
    #[serde(rename = "trustTags", skip_serializing_if = "Option::is_none")]
    pub trust_tags: Option<Vec<String>>,
    #[serde(rename = "worldFavoriteLists", skip_serializing_if = "Option::is_none")]
    pub world_favorite_lists: Option<Vec<serde_json::Value>>,
}

impl PublicProfile {
    pub fn new() -> PublicProfile {
        PublicProfile {
            age_verification_status: None,
            age_verified: None,
            background_texture_id: None,
            background_type: None,
            badges: None,
            banner_color: None,
            banner_type: None,
            banner_url: None,
            bio: None,
            bio_links: None,
            display_name: None,
            groups: None,
            has_vrc_plus: None,
            icon_frame: None,
            icon_url: None,
            id: None,
            is_economy_creator: None,
            languages: None,
            nameplate_effect: None,
            profile_effect: None,
            pronouns: None,
            public_worlds: None,
            represented_group: None,
            theme_button_color: None,
            theme_icon_color: None,
            theme_id: None,
            theme_subtext_color: None,
            total_public_worlds_count: None,
            trust_tags: None,
            world_favorite_lists: None,
        }
    }
}
