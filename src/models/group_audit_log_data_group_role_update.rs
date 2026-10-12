use crate::models;
use serde::{Deserialize, Serialize};

/// Carries only the fields the update changed.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogDataGroupRoleUpdate {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<models::GroupAuditLogStringChange>,
    #[serde(rename = "isAddedOnJoin", skip_serializing_if = "Option::is_none")]
    pub is_added_on_join: Option<models::GroupAuditLogBooleanChange>,
    #[serde(rename = "isSelfAssignable", skip_serializing_if = "Option::is_none")]
    pub is_self_assignable: Option<models::GroupAuditLogBooleanChange>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<models::GroupAuditLogStringChange>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<models::GroupAuditLogIntegerChange>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<models::GroupAuditLogStringListChange>,
    #[serde(rename = "requiresTwoFactor", skip_serializing_if = "Option::is_none")]
    pub requires_two_factor: Option<models::GroupAuditLogBooleanChange>,
}

impl GroupAuditLogDataGroupRoleUpdate {
    /// Carries only the fields the update changed.
    pub fn new() -> GroupAuditLogDataGroupRoleUpdate {
        GroupAuditLogDataGroupRoleUpdate {
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
