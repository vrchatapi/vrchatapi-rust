use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupRoleTemplateValues {
    #[serde(rename = "basePermissions")]
    pub base_permissions: Vec<models::GroupPermissions>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "roles")]
    pub roles: Vec<models::GroupRoleTemplateRole>,
}

impl GroupRoleTemplateValues {
    pub fn new(
        base_permissions: Vec<models::GroupPermissions>,
        description: String,
        name: String,
        roles: Vec<models::GroupRoleTemplateRole>,
    ) -> GroupRoleTemplateValues {
        GroupRoleTemplateValues {
            base_permissions,
            description,
            name,
            roles,
        }
    }
}
