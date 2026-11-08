use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketUserUpdateUser {
    #[serde(rename = "bio")]
    pub bio: String,
    #[serde(rename = "currentAvatar")]
    pub current_avatar: String,
    #[serde(rename = "currentAvatarImageUrl")]
    pub current_avatar_image_url: String,
    #[serde(rename = "currentAvatarThumbnailImageUrl")]
    pub current_avatar_thumbnail_image_url: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "fallbackAvatar")]
    pub fallback_avatar: String,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "profilePicOverride")]
    pub profile_pic_override: String,
    #[serde(rename = "status")]
    pub status: models::UserStatus,
    #[serde(rename = "statusDescription")]
    pub status_description: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "userIcon")]
    pub user_icon: String,
    #[serde(rename = "username")]
    pub username: String,
}

impl WebsocketUserUpdateUser {
    pub fn new(
        bio: String,
        current_avatar: String,
        current_avatar_image_url: String,
        current_avatar_thumbnail_image_url: String,
        display_name: String,
        fallback_avatar: String,
        id: String,
        profile_pic_override: String,
        status: models::UserStatus,
        status_description: String,
        tags: Vec<String>,
        user_icon: String,
        username: String,
    ) -> WebsocketUserUpdateUser {
        WebsocketUserUpdateUser {
            bio,
            current_avatar,
            current_avatar_image_url,
            current_avatar_thumbnail_image_url,
            display_name,
            fallback_avatar,
            id,
            profile_pic_override,
            status,
            status_description,
            tags,
            user_icon,
            username,
        }
    }
}
