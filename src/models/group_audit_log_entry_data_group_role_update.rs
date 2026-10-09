use crate::models;
use serde::{Deserialize, Serialize};

/// Carries only the fields the update changed.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupRoleUpdate {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<models::GroupAuditLogEntryStringChange>,
    #[serde(rename = "isAddedOnJoin", skip_serializing_if = "Option::is_none")]
    pub is_added_on_join: Option<models::GroupAuditLogEntryBooleanChange>,
    #[serde(rename = "isSelfAssignable", skip_serializing_if = "Option::is_none")]
    pub is_self_assignable: Option<models::GroupAuditLogEntryBooleanChange>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<models::GroupAuditLogEntryStringChange>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<models::GroupAuditLogEntryIntegerChange>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<models::GroupAuditLogEntryStringListChange>,
}

impl GroupAuditLogEntryDataGroupRoleUpdate {
    /// Carries only the fields the update changed.
    pub fn new() -> GroupAuditLogEntryDataGroupRoleUpdate {
        GroupAuditLogEntryDataGroupRoleUpdate {
            description: None,
            is_added_on_join: None,
            is_self_assignable: None,
            name: None,
            order: None,
            permissions: None,
        }
    }
}
