use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupRoleDelete {
    /// The role description.
    #[serde(rename = "description")]
    pub description: String,
    /// Whether the role is automatically assigned on join.
    #[serde(rename = "isAddedOnJoin")]
    pub is_added_on_join: bool,
    /// Whether users can self-assign this role.
    #[serde(rename = "isSelfAssignable")]
    pub is_self_assignable: bool,
    /// The role name.
    #[serde(rename = "name")]
    pub name: String,
    /// The display order of the role.
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// The permissions assigned to this role.
    #[serde(rename = "permissions")]
    pub permissions: Vec<models::GroupPermissions>,
    /// Whether the role requires a purchase.
    #[serde(rename = "requiresPurchase")]
    pub requires_purchase: bool,
    /// Whether the role requires two-factor authentication.
    #[serde(rename = "requiresTwoFactor")]
    pub requires_two_factor: bool,
    /// The creation timestamp of the role.
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// Whether the role is the group's default role.
    #[serde(rename = "defaultRole")]
    pub default_role: bool,
    /// Whether the role is a management role.
    #[serde(rename = "isManagementRole")]
    pub is_management_role: bool,
}

impl GroupAuditLogEntryDataGroupRoleDelete {
    pub fn new(
        description: String,
        is_added_on_join: bool,
        is_self_assignable: bool,
        name: String,
        permissions: Vec<models::GroupPermissions>,
        requires_purchase: bool,
        requires_two_factor: bool,
        created_at: chrono::DateTime<chrono::FixedOffset>,
        default_role: bool,
        is_management_role: bool,
    ) -> GroupAuditLogEntryDataGroupRoleDelete {
        GroupAuditLogEntryDataGroupRoleDelete {
            description,
            is_added_on_join,
            is_self_assignable,
            name,
            order: None,
            permissions,
            requires_purchase,
            requires_two_factor,
            created_at,
            default_role,
            is_management_role,
        }
    }
}
