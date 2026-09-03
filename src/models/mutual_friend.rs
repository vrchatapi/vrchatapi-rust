use crate::models;
use serde::{Deserialize, Serialize};

/// MutualFriend : User object received when querying mutual friends
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MutualFriend {
    /// When profilePicOverride is not empty, use it instead.
    #[serde(rename = "avatarThumbnail", skip_serializing_if = "Option::is_none")]
    pub avatar_thumbnail: Option<String>,
    /// Hex colour without a leading `#`.
    #[serde(rename = "bannerColor", skip_serializing_if = "Option::is_none")]
    pub banner_color: Option<String>,
    #[serde(rename = "bannerType", skip_serializing_if = "Option::is_none")]
    pub banner_type: Option<String>,
    /// When profilePicOverride is not empty, use it instead.
    #[serde(rename = "currentAvatarImageUrl")]
    pub current_avatar_image_url: String,
    #[serde(rename = "currentAvatarTags", skip_serializing_if = "Option::is_none")]
    pub current_avatar_tags: Option<Vec<String>>,
    /// When profilePicOverride is not empty, use it instead.
    #[serde(
        rename = "currentAvatarThumbnailImageUrl",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_avatar_thumbnail_image_url: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "iconFrame", skip_serializing_if = "Option::is_none")]
    pub icon_frame: Option<String>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    #[serde(rename = "nameplateEffect", skip_serializing_if = "Option::is_none")]
    pub nameplate_effect: Option<String>,
    #[serde(rename = "profileEffect", skip_serializing_if = "Option::is_none")]
    pub profile_effect: Option<String>,
    #[serde(rename = "profilePicOverride", skip_serializing_if = "Option::is_none")]
    pub profile_pic_override: Option<String>,
    #[serde(rename = "status")]
    pub status: models::UserStatus,
    #[serde(rename = "statusDescription")]
    pub status_description: String,
}

impl MutualFriend {
    /// User object received when querying mutual friends
    pub fn new(
        current_avatar_image_url: String,
        display_name: String,
        id: String,
        image_url: String,
        status: models::UserStatus,
        status_description: String,
    ) -> MutualFriend {
        MutualFriend {
            avatar_thumbnail: None,
            banner_color: None,
            banner_type: None,
            current_avatar_image_url,
            current_avatar_tags: None,
            current_avatar_thumbnail_image_url: None,
            display_name,
            icon_frame: None,
            icon_url: None,
            id,
            image_url,
            nameplate_effect: None,
            profile_effect: None,
            profile_pic_override: None,
            status,
            status_description,
        }
    }
}
