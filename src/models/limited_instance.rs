use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LimitedInstance {
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "capacity")]
    pub capacity: i32,
    #[serde(rename = "categoryId", deserialize_with = "Option::deserialize")]
    pub category_id: Option<String>,
    #[serde(rename = "creationLanguages")]
    pub creation_languages: Vec<serde_json::Value>,
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    #[serde(rename = "disabledPropAbilities")]
    pub disabled_prop_abilities: Vec<serde_json::Value>,
    #[serde(rename = "displayName", deserialize_with = "Option::deserialize")]
    pub display_name: Option<String>,
    #[serde(rename = "displayVibeId", deserialize_with = "Option::deserialize")]
    pub display_vibe_id: Option<String>,
    #[serde(rename = "dominantLanguage")]
    pub dominant_language: String,
    #[serde(rename = "full")]
    pub full: bool,
    #[serde(rename = "groupAccessType", skip_serializing_if = "Option::is_none")]
    pub group_access_type: Option<models::GroupAccessType>,
    /// InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance.
    #[serde(rename = "id")]
    pub id: String,
    /// InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance.
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    #[serde(rename = "languageRatio")]
    pub language_ratio: std::collections::HashMap<String, serde_json::Value>,
    /// The keys of languageRatio, ordered by their share of the instance.
    #[serde(rename = "languages")]
    pub languages: Vec<String>,
    #[serde(rename = "languagesIso639")]
    pub languages_iso639: Vec<String>,
    /// Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list.
    #[serde(rename = "location")]
    pub location: String,
    #[serde(
        rename = "minimumAvatarPerformance",
        deserialize_with = "Option::deserialize"
    )]
    pub minimum_avatar_performance: Option<String>,
    #[serde(rename = "n_users")]
    pub n_users: i32,
    /// A groupId if the instance type is \"group\", null if instance type is public, or a userId otherwise
    #[serde(rename = "ownerId", deserialize_with = "Option::deserialize")]
    pub owner_id: Option<String>,
    #[serde(rename = "permanent")]
    pub permanent: bool,
    #[serde(rename = "photonRegion")]
    pub photon_region: models::Region,
    #[serde(rename = "platforms")]
    pub platforms: models::InstancePlatforms,
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
    #[serde(rename = "shortName", deserialize_with = "Option::deserialize")]
    pub short_name: Option<String>,
    /// The tags array on Instances usually contain the language tags of the people in the instance.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "type")]
    pub r#type: models::InstanceType,
    #[serde(rename = "userCount")]
    pub user_count: i32,
    #[serde(rename = "userIcons")]
    pub user_icons: Vec<String>,
    #[serde(rename = "vibeIds")]
    pub vibe_ids: Vec<String>,
    #[serde(rename = "world")]
    pub world: models::World,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "worldId")]
    pub world_id: String,
}

impl LimitedInstance {
    pub fn new(
        active: bool,
        capacity: i32,
        category_id: Option<String>,
        creation_languages: Vec<serde_json::Value>,
        description: Option<String>,
        disabled_prop_abilities: Vec<serde_json::Value>,
        display_name: Option<String>,
        display_vibe_id: Option<String>,
        dominant_language: String,
        full: bool,
        id: String,
        instance_id: String,
        language_ratio: std::collections::HashMap<String, serde_json::Value>,
        languages: Vec<String>,
        languages_iso639: Vec<String>,
        location: String,
        minimum_avatar_performance: Option<String>,
        n_users: i32,
        owner_id: Option<String>,
        permanent: bool,
        photon_region: models::Region,
        platforms: models::InstancePlatforms,
        queue_enabled: bool,
        queue_size: i32,
        recommended_capacity: i32,
        region: models::InstanceRegion,
        short_name: Option<String>,
        tags: Vec<String>,
        r#type: models::InstanceType,
        user_count: i32,
        user_icons: Vec<String>,
        vibe_ids: Vec<String>,
        world: models::World,
        world_id: String,
    ) -> LimitedInstance {
        LimitedInstance {
            active,
            capacity,
            category_id,
            creation_languages,
            description,
            disabled_prop_abilities,
            display_name,
            display_vibe_id,
            dominant_language,
            full,
            group_access_type: None,
            id,
            instance_id,
            language_ratio,
            languages,
            languages_iso639,
            location,
            minimum_avatar_performance,
            n_users,
            owner_id,
            permanent,
            photon_region,
            platforms,
            queue_enabled,
            queue_size,
            recommended_capacity,
            region,
            role_restricted: None,
            short_name,
            tags,
            r#type,
            user_count,
            user_icons,
            vibe_ids,
            world,
            world_id,
        }
    }
}
