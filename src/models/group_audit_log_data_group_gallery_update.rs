use crate::models;
use serde::{Deserialize, Serialize};

/// Carries only the fields the update changed.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogDataGroupGalleryUpdate {
    #[serde(rename = "membersOnly", skip_serializing_if = "Option::is_none")]
    pub members_only: Option<models::GroupAuditLogBooleanChange>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<models::GroupAuditLogStringChange>,
}

impl GroupAuditLogDataGroupGalleryUpdate {
    /// Carries only the fields the update changed.
    pub fn new() -> GroupAuditLogDataGroupGalleryUpdate {
        GroupAuditLogDataGroupGalleryUpdate {
            members_only: None,
            name: None,
        }
    }
}
