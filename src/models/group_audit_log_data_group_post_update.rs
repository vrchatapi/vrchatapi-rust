use crate::models;
use serde::{Deserialize, Serialize};

/// Carries only the fields the update changed.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogDataGroupPostUpdate {
    #[serde(rename = "editorId", skip_serializing_if = "Option::is_none")]
    pub editor_id: Option<models::GroupAuditLogUserIdChange>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<models::GroupAuditLogStringChange>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<models::GroupAuditLogStringChange>,
}

impl GroupAuditLogDataGroupPostUpdate {
    /// Carries only the fields the update changed.
    pub fn new() -> GroupAuditLogDataGroupPostUpdate {
        GroupAuditLogDataGroupPostUpdate {
            editor_id: None,
            text: None,
            title: None,
        }
    }
}
