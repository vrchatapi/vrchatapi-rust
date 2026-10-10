use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateGroupRoleRequest {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "isAddedOnJoin", skip_serializing_if = "Option::is_none")]
    pub is_added_on_join: Option<bool>,
    #[serde(rename = "isSelfAssignable", skip_serializing_if = "Option::is_none")]
    pub is_self_assignable: Option<bool>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<models::GroupPermissions>>,
    #[serde(rename = "requiresPurchase", skip_serializing_if = "Option::is_none")]
    pub requires_purchase: Option<bool>,
    #[serde(rename = "requiresTwoFactor", skip_serializing_if = "Option::is_none")]
    pub requires_two_factor: Option<bool>,
}

impl CreateGroupRoleRequest {
    pub fn new(name: String) -> CreateGroupRoleRequest {
        CreateGroupRoleRequest {
            description: None,
            is_added_on_join: None,
            is_self_assignable: None,
            name,
            permissions: None,
            requires_purchase: None,
            requires_two_factor: None,
        }
    }
}
