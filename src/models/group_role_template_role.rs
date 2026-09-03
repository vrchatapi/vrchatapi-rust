use crate::models;
use serde::{Deserialize, Serialize};

/// GroupRoleTemplateRole : A role a group role template creates alongside the everyone role.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupRoleTemplateRole {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "isAddedOnJoin", skip_serializing_if = "Option::is_none")]
    pub is_added_on_join: Option<bool>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "permissions")]
    pub permissions: Vec<models::GroupPermissions>,
}

impl GroupRoleTemplateRole {
    /// A role a group role template creates alongside the everyone role.
    pub fn new(
        description: String,
        name: String,
        permissions: Vec<models::GroupPermissions>,
    ) -> GroupRoleTemplateRole {
        GroupRoleTemplateRole {
            description,
            is_added_on_join: None,
            name,
            permissions,
        }
    }
}
