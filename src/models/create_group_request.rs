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
pub struct CreateGroupRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "shortCode")]
    pub short_code: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "joinState", skip_serializing_if = "Option::is_none")]
    pub join_state: Option<models::GroupJoinState>,
    #[serde(
        rename = "iconId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_id: Option<Option<String>>,
    #[serde(
        rename = "bannerId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub banner_id: Option<Option<String>>,
    #[serde(rename = "privacy", skip_serializing_if = "Option::is_none")]
    pub privacy: Option<models::GroupPrivacy>,
    #[serde(rename = "roleTemplate")]
    pub role_template: models::GroupRoleTemplate,
}

impl CreateGroupRequest {
    pub fn new(
        name: String,
        short_code: String,
        role_template: models::GroupRoleTemplate,
    ) -> CreateGroupRequest {
        CreateGroupRequest {
            name,
            short_code,
            description: None,
            join_state: None,
            icon_id: None,
            banner_id: None,
            privacy: None,
            role_template,
        }
    }
}
