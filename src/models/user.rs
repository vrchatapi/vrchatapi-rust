use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(
        rename = "acceptedPrivacyVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub accepted_privacy_version: Option<i32>,
    #[serde(rename = "acceptedTOSVersion", skip_serializing_if = "Option::is_none")]
    pub accepted_tos_version: Option<i32>,
    #[serde(
        rename = "accountDeletionDate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_deletion_date: Option<Option<String>>,
    #[serde(
        rename = "accountDeletionLog",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_deletion_log: Option<Option<Vec<serde_json::Value>>>,
    #[serde(rename = "ageVerificationStatus")]
    pub age_verification_status: models::AgeVerificationStatus,
    /// `true` if, user is age verified (not 18+).
    #[serde(rename = "ageVerified")]
    pub age_verified: bool,
    #[serde(rename = "allowAvatarCopying")]
    pub allow_avatar_copying: bool,
    #[serde(rename = "appleDetails", skip_serializing_if = "Option::is_none")]
    pub apple_details: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "badges", skip_serializing_if = "Option::is_none")]
    pub badges: Option<Vec<models::Badge>>,
    #[serde(rename = "bannerColor", skip_serializing_if = "Option::is_none")]
    pub banner_color: Option<String>,
    #[serde(rename = "bannerType", skip_serializing_if = "Option::is_none")]
    pub banner_type: Option<String>,
    #[serde(rename = "bannerUrl", skip_serializing_if = "Option::is_none")]
    pub banner_url: Option<String>,
    #[serde(rename = "bio")]
    pub bio: String,
    #[serde(rename = "bioLinks")]
    pub bio_links: Vec<String>,
    /// When profilePicOverride is not empty, use it instead.
    #[serde(rename = "currentAvatarImageUrl")]
    pub current_avatar_image_url: String,
    #[serde(rename = "currentAvatarTags")]
    pub current_avatar_tags: Vec<String>,
    /// When profilePicOverride is not empty, use it instead.
    #[serde(rename = "currentAvatarThumbnailImageUrl")]
    pub current_avatar_thumbnail_image_url: String,
    #[serde(rename = "date_joined")]
    pub date_joined: chrono::NaiveDate,
    #[serde(rename = "developerType")]
    pub developer_type: models::DeveloperType,
    /// A users visual display name. This is what shows up in-game, and can different from their `username`. Changing display name is restricted to a cooldown period.
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "friendKey")]
    pub friend_key: String,
    /// State of a friend request between the caller and this user. VRChat sends the string `\"null\"`, not JSON `null`.
    #[serde(
        rename = "friendRequestStatus",
        skip_serializing_if = "Option::is_none"
    )]
    pub friend_request_status: Option<String>,
    #[serde(rename = "iconFrame", skip_serializing_if = "Option::is_none")]
    pub icon_frame: Option<String>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "id")]
    pub id: String,
    /// InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance.
    #[serde(rename = "instanceId", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "isEconomyCreator", skip_serializing_if = "Option::is_none")]
    pub is_economy_creator: Option<bool>,
    /// Either their `friendKey`, or empty string if you are not friends. Unknown usage.
    #[serde(rename = "isFriend")]
    pub is_friend: bool,
    /// Either a date-time or empty string.
    #[serde(rename = "last_activity")]
    pub last_activity: String,
    /// Either a date-time or empty string.
    #[serde(rename = "last_login")]
    pub last_login: String,
    #[serde(
        rename = "last_mobile",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_mobile: Option<Option<String>>,
    /// This is normally `android`, `ios`, `standalonewindows`, `web`, or the empty value ``, but also supposedly can be any random Unity version such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`.
    #[serde(rename = "last_platform")]
    pub last_platform: String,
    /// Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "nameplateEffect", skip_serializing_if = "Option::is_none")]
    pub nameplate_effect: Option<String>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "profileEffect", skip_serializing_if = "Option::is_none")]
    pub profile_effect: Option<String>,
    #[serde(rename = "profilePicOverride")]
    pub profile_pic_override: String,
    #[serde(rename = "profilePicOverrideThumbnail")]
    pub profile_pic_override_thumbnail: String,
    #[serde(rename = "pronouns")]
    pub pronouns: String,
    #[serde(rename = "state")]
    pub state: models::UserState,
    #[serde(rename = "status")]
    pub status: models::UserStatus,
    #[serde(rename = "statusDescription")]
    pub status_description: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(
        rename = "travelingToInstance",
        skip_serializing_if = "Option::is_none"
    )]
    pub traveling_to_instance: Option<String>,
    #[serde(
        rename = "travelingToLocation",
        skip_serializing_if = "Option::is_none"
    )]
    pub traveling_to_location: Option<String>,
    #[serde(rename = "travelingToWorld", skip_serializing_if = "Option::is_none")]
    pub traveling_to_world: Option<String>,
    #[serde(rename = "userIcon")]
    pub user_icon: String,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "worldId", skip_serializing_if = "Option::is_none")]
    pub world_id: Option<String>,
}

impl User {
    pub fn new(
        age_verification_status: models::AgeVerificationStatus,
        age_verified: bool,
        allow_avatar_copying: bool,
        bio: String,
        bio_links: Vec<String>,
        current_avatar_image_url: String,
        current_avatar_tags: Vec<String>,
        current_avatar_thumbnail_image_url: String,
        date_joined: chrono::NaiveDate,
        developer_type: models::DeveloperType,
        display_name: String,
        friend_key: String,
        id: String,
        is_friend: bool,
        last_activity: String,
        last_login: String,
        last_platform: String,
        profile_pic_override: String,
        profile_pic_override_thumbnail: String,
        pronouns: String,
        state: models::UserState,
        status: models::UserStatus,
        status_description: String,
        tags: Vec<String>,
        user_icon: String,
    ) -> User {
        User {
            accepted_privacy_version: None,
            accepted_tos_version: None,
            account_deletion_date: None,
            account_deletion_log: None,
            age_verification_status,
            age_verified,
            allow_avatar_copying,
            apple_details: None,
            badges: None,
            banner_color: None,
            banner_type: None,
            banner_url: None,
            bio,
            bio_links,
            current_avatar_image_url,
            current_avatar_tags,
            current_avatar_thumbnail_image_url,
            date_joined,
            developer_type,
            display_name,
            friend_key,
            friend_request_status: None,
            icon_frame: None,
            icon_url: None,
            id,
            instance_id: None,
            is_economy_creator: None,
            is_friend,
            last_activity,
            last_login,
            last_mobile: None,
            last_platform,
            location: None,
            nameplate_effect: None,
            note: None,
            platform: None,
            profile_effect: None,
            profile_pic_override,
            profile_pic_override_thumbnail,
            pronouns,
            state,
            status,
            status_description,
            tags,
            traveling_to_instance: None,
            traveling_to_location: None,
            traveling_to_world: None,
            user_icon,
            world_id: None,
        }
    }
}
