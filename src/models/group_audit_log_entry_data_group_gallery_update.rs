use crate::models;
use serde::{Deserialize, Serialize};

/// Carries only the fields the update changed.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupGalleryUpdate {
    #[serde(rename = "membersOnly", skip_serializing_if = "Option::is_none")]
    pub members_only: Option<models::GroupAuditLogEntryBooleanChange>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<models::GroupAuditLogEntryStringChange>,
}

impl GroupAuditLogEntryDataGroupGalleryUpdate {
    /// Carries only the fields the update changed.
    pub fn new() -> GroupAuditLogEntryDataGroupGalleryUpdate {
        GroupAuditLogEntryDataGroupGalleryUpdate {
            members_only: None,
            name: None,
        }
    }
}
