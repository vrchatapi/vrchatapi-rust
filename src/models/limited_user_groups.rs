/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LimitedUserGroups {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "shortCode", skip_serializing_if = "Option::is_none")]
    pub short_code: Option<String>,
    #[serde(rename = "discriminator", skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "iconId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_id: Option<Option<String>>,
    #[serde(
        rename = "iconUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_url: Option<Option<String>>,
    #[serde(
        rename = "bannerId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub banner_id: Option<Option<String>>,
    #[serde(
        rename = "bannerUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub banner_url: Option<Option<String>>,
    #[serde(rename = "privacy", skip_serializing_if = "Option::is_none")]
    pub privacy: Option<String>,
    #[serde(
        rename = "lastPostCreatedAt",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_post_created_at: Option<Option<String>>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "ownerId", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "memberCount", skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "memberVisibility", skip_serializing_if = "Option::is_none")]
    pub member_visibility: Option<String>,
    #[serde(rename = "isRepresenting", skip_serializing_if = "Option::is_none")]
    pub is_representing: Option<bool>,
    #[serde(rename = "mutualGroup", skip_serializing_if = "Option::is_none")]
    pub mutual_group: Option<bool>,
    #[serde(
        rename = "lastPostReadAt",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_post_read_at: Option<Option<String>>,
}

impl LimitedUserGroups {
    pub fn new() -> LimitedUserGroups {
        LimitedUserGroups {
            id: None,
            name: None,
            short_code: None,
            discriminator: None,
            description: None,
            icon_id: None,
            icon_url: None,
            banner_id: None,
            banner_url: None,
            privacy: None,
            last_post_created_at: None,
            owner_id: None,
            member_count: None,
            group_id: None,
            member_visibility: None,
            is_representing: None,
            mutual_group: None,
            last_post_read_at: None,
        }
    }
}
