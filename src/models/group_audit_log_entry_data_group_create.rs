use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupCreate {
    #[serde(rename = "bannerId", deserialize_with = "Option::deserialize")]
    pub banner_id: Option<String>,
    #[serde(rename = "bannerVersion", skip_serializing_if = "Option::is_none")]
    pub banner_version: Option<i32>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "discoverability")]
    pub discoverability: models::GroupAuditLogEntryDataGroupCreateDiscoverability,
    #[serde(rename = "galleries", skip_serializing_if = "Option::is_none")]
    pub galleries: Option<Vec<models::GroupAuditLogEntryDataGroupCreateGallery>>,
    #[serde(rename = "iconId", deserialize_with = "Option::deserialize")]
    pub icon_id: Option<String>,
    #[serde(rename = "iconVersion", skip_serializing_if = "Option::is_none")]
    pub icon_version: Option<i32>,
    #[serde(rename = "joinState")]
    pub join_state: models::GroupJoinState,
    #[serde(rename = "name")]
    pub name: String,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "privacy")]
    pub privacy: models::GroupPrivacy,
    #[serde(rename = "rules")]
    pub rules: String,
    #[serde(rename = "shortCode")]
    pub short_code: String,
}

impl GroupAuditLogEntryDataGroupCreate {
    pub fn new(
        banner_id: Option<String>,
        description: String,
        discoverability: models::GroupAuditLogEntryDataGroupCreateDiscoverability,
        icon_id: Option<String>,
        join_state: models::GroupJoinState,
        name: String,
        owner_id: String,
        privacy: models::GroupPrivacy,
        rules: String,
        short_code: String,
    ) -> GroupAuditLogEntryDataGroupCreate {
        GroupAuditLogEntryDataGroupCreate {
            banner_id,
            banner_version: None,
            description,
            discoverability,
            galleries: None,
            icon_id,
            icon_version: None,
            join_state,
            name,
            owner_id,
            privacy,
            rules,
            short_code,
        }
    }
}
