use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateGroupRoleRequest {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "isAddedOnJoin", skip_serializing_if = "Option::is_none")]
    pub is_added_on_join: Option<bool>,
    #[serde(rename = "isSelfAssignable", skip_serializing_if = "Option::is_none")]
    pub is_self_assignable: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<models::GroupPermissions>>,
    #[serde(rename = "requiresTwoFactor", skip_serializing_if = "Option::is_none")]
    pub requires_two_factor: Option<bool>,
}

impl UpdateGroupRoleRequest {
    pub fn new() -> UpdateGroupRoleRequest {
        UpdateGroupRoleRequest {
            description: None,
            is_added_on_join: None,
            is_self_assignable: None,
            name: None,
            order: None,
            permissions: None,
            requires_two_factor: None,
        }
    }
}
