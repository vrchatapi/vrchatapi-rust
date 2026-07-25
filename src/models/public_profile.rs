use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicProfile {
    #[serde(rename = "ageVerificationStatus")]
    pub age_verification_status: models::AgeVerificationStatus,
    /// `true` if, user is age verified (not 18+).
    #[serde(rename = "ageVerified")]
    pub age_verified: bool,
    #[serde(rename = "backgroundType")]
    pub background_type: String,
    #[serde(rename = "badges")]
    pub badges: Vec<models::Badge>,
    #[serde(rename = "bannerColor")]
    pub banner_color: String,
    #[serde(rename = "bannerType")]
    pub banner_type: String,
    #[serde(rename = "bio")]
    pub bio: String,
    #[serde(rename = "bioLinks")]
    pub bio_links: Vec<String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "hasVrcPlus")]
    pub has_vrc_plus: bool,
    #[serde(rename = "iconFrame")]
    pub icon_frame: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isEconomyCreator")]
    pub is_economy_creator: bool,
    #[serde(rename = "languages")]
    pub languages: Vec<String>,
    #[serde(rename = "nameplateEffect")]
    pub nameplate_effect: String,
    #[serde(rename = "profileEffect")]
    pub profile_effect: String,
    #[serde(rename = "pronouns")]
    pub pronouns: String,
    #[serde(rename = "representedGroup", deserialize_with = "Option::deserialize")]
    pub represented_group: Option<models::ProfileRepresentedGroup>,
    #[serde(rename = "themeId")]
    pub theme_id: String,
    #[serde(rename = "trustTags")]
    pub trust_tags: Vec<String>,
}

impl PublicProfile {
    pub fn new(
        age_verification_status: models::AgeVerificationStatus,
        age_verified: bool,
        background_type: String,
        badges: Vec<models::Badge>,
        banner_color: String,
        banner_type: String,
        bio: String,
        bio_links: Vec<String>,
        display_name: String,
        has_vrc_plus: bool,
        icon_frame: String,
        icon_url: String,
        id: String,
        is_economy_creator: bool,
        languages: Vec<String>,
        nameplate_effect: String,
        profile_effect: String,
        pronouns: String,
        represented_group: Option<models::ProfileRepresentedGroup>,
        theme_id: String,
        trust_tags: Vec<String>,
    ) -> PublicProfile {
        PublicProfile {
            age_verification_status,
            age_verified,
            background_type,
            badges,
            banner_color,
            banner_type,
            bio,
            bio_links,
            display_name,
            has_vrc_plus,
            icon_frame,
            icon_url,
            id,
            is_economy_creator,
            languages,
            nameplate_effect,
            profile_effect,
            pronouns,
            represented_group,
            theme_id,
            trust_tags,
        }
    }
}
