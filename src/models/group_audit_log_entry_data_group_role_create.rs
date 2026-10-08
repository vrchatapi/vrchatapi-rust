use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupRoleCreate {
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
    /// The group ID.
    #[serde(rename = "groupId")]
    pub group_id: String,
    /// The ID of the user who last updated the role.
    #[serde(rename = "lastUpdatedByUserId")]
    pub last_updated_by_user_id: String,
}

impl GroupAuditLogEntryDataGroupRoleCreate {
    pub fn new(
        description: String,
        is_added_on_join: bool,
        is_self_assignable: bool,
        name: String,
        permissions: Vec<models::GroupPermissions>,
        requires_purchase: bool,
        requires_two_factor: bool,
        group_id: String,
        last_updated_by_user_id: String,
    ) -> GroupAuditLogEntryDataGroupRoleCreate {
        GroupAuditLogEntryDataGroupRoleCreate {
            description,
            is_added_on_join,
            is_self_assignable,
            name,
            order: None,
            permissions,
            requires_purchase,
            requires_two_factor,
            group_id,
            last_updated_by_user_id,
        }
    }
}
