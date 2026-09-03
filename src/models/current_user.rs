use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentUser {
    #[serde(
        rename = "acceptedPrivacyVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub accepted_privacy_version: Option<i32>,
    #[serde(rename = "acceptedTOSVersion")]
    pub accepted_tos_version: i32,
    #[serde(
        rename = "accountDeletionDate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_deletion_date: Option<Option<chrono::NaiveDate>>,
    #[serde(
        rename = "accountDeletionLog",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_deletion_log: Option<Option<Vec<models::AccountDeletionLog>>>,
    #[serde(rename = "activeFriends", skip_serializing_if = "Option::is_none")]
    pub active_friends: Option<Vec<String>>,
    #[serde(rename = "ageVerificationStatus")]
    pub age_verification_status: models::AgeVerificationStatus,
    /// `true` if, user is age verified (not 18+).
    #[serde(rename = "ageVerified")]
    pub age_verified: bool,
    #[serde(rename = "allowAvatarCopying")]
    pub allow_avatar_copying: bool,
    #[serde(rename = "appleDetails", skip_serializing_if = "Option::is_none")]
    pub apple_details: Option<serde_json::Value>,
    #[serde(rename = "appleId", skip_serializing_if = "Option::is_none")]
    pub apple_id: Option<String>,
    /// The auth token for NEWLY REGISTERED ACCOUNTS ONLY (/auth/register)
    #[serde(rename = "authToken", skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(rename = "badges", skip_serializing_if = "Option::is_none")]
    pub badges: Option<Vec<models::Badge>>,
    #[serde(rename = "bannerColor", skip_serializing_if = "Option::is_none")]
    pub banner_color: Option<String>,
    #[serde(rename = "bannerType", skip_serializing_if = "Option::is_none")]
    pub banner_type: Option<String>,
    #[serde(rename = "bio")]
    pub bio: String,
    #[serde(rename = "bioLinks")]
    pub bio_links: Vec<String>,
    #[serde(rename = "completedTutorials", skip_serializing_if = "Option::is_none")]
    pub completed_tutorials: Option<Vec<String>>,
    /// These tags begin with `content_` and control content gating
    #[serde(rename = "contentFilters", skip_serializing_if = "Option::is_none")]
    pub content_filters: Option<Vec<String>>,
    #[serde(rename = "currentAvatar")]
    pub current_avatar: String,
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
    #[serde(rename = "discordDetails", skip_serializing_if = "Option::is_none")]
    pub discord_details: Option<models::DiscordDetails>,
    /// https://discord.com/developers/docs/reference#snowflakes
    #[serde(rename = "discordId", skip_serializing_if = "Option::is_none")]
    pub discord_id: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    #[serde(rename = "fallbackAvatar", skip_serializing_if = "Option::is_none")]
    pub fallback_avatar: Option<String>,
    /// Always empty array.
    #[serde(rename = "friendGroupNames")]
    pub friend_group_names: Vec<String>,
    #[serde(rename = "friendKey")]
    pub friend_key: String,
    /// State of a friend request between the caller and this user. VRChat sends the string `\"null\"`, not JSON `null`.
    #[serde(
        rename = "friendRequestStatus",
        skip_serializing_if = "Option::is_none"
    )]
    pub friend_request_status: Option<String>,
    #[serde(rename = "friends")]
    pub friends: Vec<String>,
    #[serde(rename = "googleDetails", skip_serializing_if = "Option::is_none")]
    pub google_details: Option<serde_json::Value>,
    #[serde(rename = "googleId", skip_serializing_if = "Option::is_none")]
    pub google_id: Option<String>,
    #[serde(rename = "hasBirthday")]
    pub has_birthday: bool,
    #[serde(
        rename = "hasDiscordFriendsOptOut",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_discord_friends_opt_out: Option<bool>,
    #[serde(rename = "hasEmail")]
    pub has_email: bool,
    #[serde(rename = "hasLoggedInFromClient")]
    pub has_logged_in_from_client: bool,
    #[serde(rename = "hasPendingEmail")]
    pub has_pending_email: bool,
    #[serde(
        rename = "hasSharedConnectionsOptOut",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_shared_connections_opt_out: Option<bool>,
    #[serde(
        rename = "hideContentFilterSettings",
        skip_serializing_if = "Option::is_none"
    )]
    pub hide_content_filter_settings: Option<bool>,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "homeLocation")]
    pub home_location: String,
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
    #[serde(rename = "isAdult")]
    pub is_adult: bool,
    #[serde(rename = "isBoopingEnabled", skip_serializing_if = "Option::is_none")]
    pub is_booping_enabled: Option<bool>,
    #[serde(rename = "isEconomyCreator", skip_serializing_if = "Option::is_none")]
    pub is_economy_creator: Option<bool>,
    #[serde(rename = "isFriend")]
    pub is_friend: bool,
    #[serde(rename = "isTemporary", skip_serializing_if = "Option::is_none")]
    pub is_temporary: Option<bool>,
    #[serde(rename = "last_activity", skip_serializing_if = "Option::is_none")]
    pub last_activity: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[serde(rename = "last_login")]
    pub last_login: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "last_mobile", deserialize_with = "Option::deserialize")]
    pub last_mobile: Option<chrono::DateTime<chrono::FixedOffset>>,
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
    #[serde(rename = "obfuscatedEmail")]
    pub obfuscated_email: String,
    #[serde(rename = "obfuscatedPendingEmail")]
    pub obfuscated_pending_email: String,
    #[serde(rename = "oculusId")]
    pub oculus_id: String,
    #[serde(rename = "offlineFriends", skip_serializing_if = "Option::is_none")]
    pub offline_friends: Option<Vec<String>>,
    #[serde(rename = "onlineFriends", skip_serializing_if = "Option::is_none")]
    pub online_friends: Option<Vec<String>>,
    #[serde(rename = "pastDisplayNames")]
    pub past_display_names: Vec<models::PastDisplayName>,
    #[serde(
        rename = "personalizationOptOut",
        skip_serializing_if = "Option::is_none"
    )]
    pub personalization_opt_out: Option<bool>,
    #[serde(rename = "picoId", skip_serializing_if = "Option::is_none")]
    pub pico_id: Option<String>,
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "platform_history", skip_serializing_if = "Option::is_none")]
    pub platform_history: Option<Vec<models::CurrentUserPlatformHistoryInner>>,
    #[serde(rename = "presence", skip_serializing_if = "Option::is_none")]
    pub presence: Option<models::CurrentUserPresence>,
    #[serde(rename = "profileEffect", skip_serializing_if = "Option::is_none")]
    pub profile_effect: Option<String>,
    #[serde(rename = "profilePicOverride")]
    pub profile_pic_override: String,
    #[serde(rename = "profilePicOverrideThumbnail")]
    pub profile_pic_override_thumbnail: String,
    #[serde(rename = "pronouns")]
    pub pronouns: String,
    #[serde(rename = "pronounsHistory")]
    pub pronouns_history: Vec<String>,
    #[serde(
        rename = "queuedInstance",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub queued_instance: Option<Option<String>>,
    #[serde(
        rename = "receiveMobileInvitations",
        skip_serializing_if = "Option::is_none"
    )]
    pub receive_mobile_invitations: Option<bool>,
    #[serde(rename = "state")]
    pub state: models::UserState,
    #[serde(rename = "status")]
    pub status: models::UserStatus,
    #[serde(rename = "statusDescription")]
    pub status_description: String,
    #[serde(rename = "statusFirstTime")]
    pub status_first_time: bool,
    #[serde(rename = "statusHistory")]
    pub status_history: Vec<String>,
    #[serde(rename = "steamDetails")]
    pub steam_details: serde_json::Value,
    #[serde(rename = "steamId")]
    pub steam_id: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(
        rename = "temporaryExpiryDate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub temporary_expiry_date: Option<Option<serde_json::Value>>,
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
    #[serde(rename = "twitchDetails", skip_serializing_if = "Option::is_none")]
    pub twitch_details: Option<serde_json::Value>,
    #[serde(rename = "twitchId", skip_serializing_if = "Option::is_none")]
    pub twitch_id: Option<String>,
    #[serde(rename = "twoFactorAuthEnabled")]
    pub two_factor_auth_enabled: bool,
    #[serde(
        rename = "twoFactorAuthEnabledDate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub two_factor_auth_enabled_date: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    #[serde(rename = "unsubscribe")]
    pub unsubscribe: bool,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[serde(rename = "userIcon")]
    pub user_icon: String,
    #[serde(
        rename = "userLanguage",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_language: Option<Option<String>>,
    #[serde(
        rename = "userLanguageCode",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_language_code: Option<Option<String>>,
    /// Your own unique name, used during login. Distinct from `displayName`, and never returned for another user.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "usesGeneratedPassword")]
    pub uses_generated_password: bool,
    #[serde(rename = "viveId", skip_serializing_if = "Option::is_none")]
    pub vive_id: Option<String>,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "worldId", skip_serializing_if = "Option::is_none")]
    pub world_id: Option<String>,
}

impl CurrentUser {
    pub fn new(
        accepted_tos_version: i32,
        age_verification_status: models::AgeVerificationStatus,
        age_verified: bool,
        allow_avatar_copying: bool,
        bio: String,
        bio_links: Vec<String>,
        current_avatar: String,
        current_avatar_image_url: String,
        current_avatar_tags: Vec<String>,
        current_avatar_thumbnail_image_url: String,
        date_joined: chrono::NaiveDate,
        developer_type: models::DeveloperType,
        display_name: String,
        email_verified: bool,
        friend_group_names: Vec<String>,
        friend_key: String,
        friends: Vec<String>,
        has_birthday: bool,
        has_email: bool,
        has_logged_in_from_client: bool,
        has_pending_email: bool,
        home_location: String,
        id: String,
        is_adult: bool,
        is_friend: bool,
        last_login: chrono::DateTime<chrono::FixedOffset>,
        last_mobile: Option<chrono::DateTime<chrono::FixedOffset>>,
        last_platform: String,
        obfuscated_email: String,
        obfuscated_pending_email: String,
        oculus_id: String,
        past_display_names: Vec<models::PastDisplayName>,
        profile_pic_override: String,
        profile_pic_override_thumbnail: String,
        pronouns: String,
        pronouns_history: Vec<String>,
        state: models::UserState,
        status: models::UserStatus,
        status_description: String,
        status_first_time: bool,
        status_history: Vec<String>,
        steam_details: serde_json::Value,
        steam_id: String,
        tags: Vec<String>,
        two_factor_auth_enabled: bool,
        unsubscribe: bool,
        user_icon: String,
        uses_generated_password: bool,
    ) -> CurrentUser {
        CurrentUser {
            accepted_privacy_version: None,
            accepted_tos_version,
            account_deletion_date: None,
            account_deletion_log: None,
            active_friends: None,
            age_verification_status,
            age_verified,
            allow_avatar_copying,
            apple_details: None,
            apple_id: None,
            auth_token: None,
            badges: None,
            banner_color: None,
            banner_type: None,
            bio,
            bio_links,
            completed_tutorials: None,
            content_filters: None,
            current_avatar,
            current_avatar_image_url,
            current_avatar_tags,
            current_avatar_thumbnail_image_url,
            date_joined,
            developer_type,
            discord_details: None,
            discord_id: None,
            display_name,
            email_verified,
            fallback_avatar: None,
            friend_group_names,
            friend_key,
            friend_request_status: None,
            friends,
            google_details: None,
            google_id: None,
            has_birthday,
            has_discord_friends_opt_out: None,
            has_email,
            has_logged_in_from_client,
            has_pending_email,
            has_shared_connections_opt_out: None,
            hide_content_filter_settings: None,
            home_location,
            icon_frame: None,
            icon_url: None,
            id,
            instance_id: None,
            is_adult,
            is_booping_enabled: None,
            is_economy_creator: None,
            is_friend,
            is_temporary: None,
            last_activity: None,
            last_login,
            last_mobile,
            last_platform,
            location: None,
            nameplate_effect: None,
            note: None,
            obfuscated_email,
            obfuscated_pending_email,
            oculus_id,
            offline_friends: None,
            online_friends: None,
            past_display_names,
            personalization_opt_out: None,
            pico_id: None,
            platform: None,
            platform_history: None,
            presence: None,
            profile_effect: None,
            profile_pic_override,
            profile_pic_override_thumbnail,
            pronouns,
            pronouns_history,
            queued_instance: None,
            receive_mobile_invitations: None,
            state,
            status,
            status_description,
            status_first_time,
            status_history,
            steam_details,
            steam_id,
            tags,
            temporary_expiry_date: None,
            traveling_to_instance: None,
            traveling_to_location: None,
            traveling_to_world: None,
            twitch_details: None,
            twitch_id: None,
            two_factor_auth_enabled,
            two_factor_auth_enabled_date: None,
            unsubscribe,
            updated_at: None,
            user_icon,
            user_language: None,
            user_language_code: None,
            username: None,
            uses_generated_password,
            vive_id: None,
            world_id: None,
        }
    }
}
