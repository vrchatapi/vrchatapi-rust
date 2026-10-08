use crate::models;
use serde::{Deserialize, Serialize};

/// GroupAuditLogEntryDataGroupPostUpdate : Carries only the fields the update changed.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupPostUpdate {
    #[serde(rename = "editorId", skip_serializing_if = "Option::is_none")]
    pub editor_id: Option<models::GroupAuditLogEntryUserIdChange>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<models::GroupAuditLogEntryStringChange>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<models::GroupAuditLogEntryStringChange>,
}

impl GroupAuditLogEntryDataGroupPostUpdate {
    /// Carries only the fields the update changed.
    pub fn new() -> GroupAuditLogEntryDataGroupPostUpdate {
        GroupAuditLogEntryDataGroupPostUpdate {
            editor_id: None,
            text: None,
            title: None,
        }
    }
}
