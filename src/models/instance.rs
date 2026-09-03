use crate::models;
use serde::{Deserialize, Serialize};

/// Instance : * `hidden` field is only present if InstanceType is `hidden` aka \"Friends+\", and is instance creator. * `friends` field is only present if InstanceType is `friends` aka \"Friends\", and is instance creator. * `private` field is only present if InstanceType is `private` aka \"Invite\" or \"Invite+\", and is instance creator.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Instance {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(
        rename = "ageGate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub age_gate: Option<Option<bool>>,
    #[serde(
        rename = "calendarEntryId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub calendar_entry_id: Option<Option<String>>,
    #[serde(rename = "canRequestInvite", skip_serializing_if = "Option::is_none")]
    pub can_request_invite: Option<bool>,
    #[serde(rename = "capacity", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(rename = "categoryId", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// Always returns \"unknown\".
    #[serde(rename = "clientNumber")]
    pub client_number: String,
    #[serde(
        rename = "closedAt",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub closed_at: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    #[serde(rename = "contentSettings", skip_serializing_if = "Option::is_none")]
    pub content_settings: Option<models::InstanceContentSettings>,
    #[serde(rename = "creationLanguages", skip_serializing_if = "Option::is_none")]
    pub creation_languages: Option<Vec<serde_json::Value>>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "creatorId", skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    #[serde(
        rename = "disabledPropAbilities",
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_prop_abilities: Option<Vec<serde_json::Value>>,
    #[serde(
        rename = "displayName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<Option<String>>,
    #[serde(rename = "dominantLanguage", skip_serializing_if = "Option::is_none")]
    pub dominant_language: Option<String>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "friends", skip_serializing_if = "Option::is_none")]
    pub friends: Option<String>,
    #[serde(rename = "full")]
    pub full: bool,
    #[serde(
        rename = "gameServerVersion",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub game_server_version: Option<Option<i32>>,
    #[serde(rename = "groupAccessType", skip_serializing_if = "Option::is_none")]
    pub group_access_type: Option<models::GroupAccessType>,
    #[serde(
        rename = "hardClose",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub hard_close: Option<Option<bool>>,
    #[serde(rename = "hasCapacityForYou", skip_serializing_if = "Option::is_none")]
    pub has_capacity_for_you: Option<bool>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "hidden", skip_serializing_if = "Option::is_none")]
    pub hidden: Option<String>,
    /// InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance.
    #[serde(rename = "id")]
    pub id: String,
    /// InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance.
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    #[serde(
        rename = "instancePersistenceEnabled",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_persistence_enabled: Option<Option<bool>>,
    #[serde(rename = "languageRatio", skip_serializing_if = "Option::is_none")]
    pub language_ratio: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The keys of languageRatio, ordered by their share of the instance.
    #[serde(rename = "languages", skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(rename = "languagesIso639", skip_serializing_if = "Option::is_none")]
    pub languages_iso639: Option<Vec<String>>,
    /// Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list.
    #[serde(rename = "location")]
    pub location: String,
    #[serde(
        rename = "minimumAvatarPerformance",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_avatar_performance: Option<Option<String>>,
    #[serde(rename = "n_users")]
    pub n_users: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nonce", skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    /// A groupId if the instance type is \"group\", null if instance type is public, or a userId otherwise
    #[serde(
        rename = "ownerId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub owner_id: Option<Option<String>>,
    #[serde(rename = "permanent")]
    pub permanent: bool,
    #[serde(rename = "photonRegion")]
    pub photon_region: models::Region,
    #[serde(rename = "platforms")]
    pub platforms: models::InstancePlatforms,
    #[serde(
        rename = "playerPersistenceEnabled",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub player_persistence_enabled: Option<Option<bool>>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "private", skip_serializing_if = "Option::is_none")]
    pub private: Option<String>,
    #[serde(rename = "queueEnabled")]
    pub queue_enabled: bool,
    #[serde(rename = "queueSize")]
    pub queue_size: i32,
    #[serde(rename = "recommendedCapacity")]
    pub recommended_capacity: i32,
    #[serde(rename = "region")]
    pub region: models::InstanceRegion,
    #[serde(rename = "roleRestricted", skip_serializing_if = "Option::is_none")]
    pub role_restricted: Option<bool>,
    #[serde(rename = "secureName")]
    pub secure_name: String,
    #[serde(
        rename = "shortName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_name: Option<Option<String>>,
    #[serde(rename = "strict")]
    pub strict: bool,
    /// The tags array on Instances usually contain the language tags of the people in the instance.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "type")]
    pub r#type: models::InstanceType,
    #[serde(rename = "userCount")]
    pub user_count: i32,
    #[serde(rename = "userIcons", skip_serializing_if = "Option::is_none")]
    pub user_icons: Option<Vec<String>>,
    /// The users field is present on instances created by the requesting user.
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<models::LimitedUserInstance>>,
    #[serde(rename = "vibeIds", skip_serializing_if = "Option::is_none")]
    pub vibe_ids: Option<Vec<String>>,
    #[serde(rename = "world")]
    pub world: models::World,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "worldId")]
    pub world_id: String,
}

impl Instance {
    /// * `hidden` field is only present if InstanceType is `hidden` aka \"Friends+\", and is instance creator. * `friends` field is only present if InstanceType is `friends` aka \"Friends\", and is instance creator. * `private` field is only present if InstanceType is `private` aka \"Invite\" or \"Invite+\", and is instance creator.
    pub fn new(
        client_number: String,
        full: bool,
        id: String,
        instance_id: String,
        location: String,
        n_users: i32,
        name: String,
        permanent: bool,
        photon_region: models::Region,
        platforms: models::InstancePlatforms,
        queue_enabled: bool,
        queue_size: i32,
        recommended_capacity: i32,
        region: models::InstanceRegion,
        secure_name: String,
        strict: bool,
        tags: Vec<String>,
        r#type: models::InstanceType,
        user_count: i32,
        world: models::World,
        world_id: String,
    ) -> Instance {
        Instance {
            active: None,
            age_gate: None,
            calendar_entry_id: None,
            can_request_invite: None,
            capacity: None,
            category_id: None,
            client_number,
            closed_at: None,
            content_settings: None,
            creation_languages: None,
            creator_id: None,
            description: None,
            disabled_prop_abilities: None,
            display_name: None,
            dominant_language: None,
            friends: None,
            full,
            game_server_version: None,
            group_access_type: None,
            hard_close: None,
            has_capacity_for_you: None,
            hidden: None,
            id,
            instance_id,
            instance_persistence_enabled: None,
            language_ratio: None,
            languages: None,
            languages_iso639: None,
            location,
            minimum_avatar_performance: None,
            n_users,
            name,
            nonce: None,
            owner_id: None,
            permanent,
            photon_region,
            platforms,
            player_persistence_enabled: None,
            private: None,
            queue_enabled,
            queue_size,
            recommended_capacity,
            region,
            role_restricted: None,
            secure_name,
            short_name: None,
            strict,
            tags,
            r#type,
            user_count,
            user_icons: None,
            users: None,
            vibe_ids: None,
            world,
            world_id,
        }
    }
}
