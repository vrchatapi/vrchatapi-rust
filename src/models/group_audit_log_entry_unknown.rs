use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryUnknown {
    #[serde(flatten)]
    pub group_audit_log_entry_base: models::GroupAuditLogEntryBase,
    #[serde(rename = "data")]
    pub data: std::collections::HashMap<String, serde_json::Value>,
    /// The type of event that occurred.
    #[serde(rename = "eventType")]
    pub event_type: String,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryUnknown {
    pub fn new(
        group_audit_log_entry_base: models::GroupAuditLogEntryBase,
        data: std::collections::HashMap<String, serde_json::Value>,
        event_type: String,
        target_id: String,
    ) -> GroupAuditLogEntryUnknown {
        GroupAuditLogEntryUnknown {
            group_audit_log_entry_base,
            data,
            event_type,
            target_id,
        }
    }
}
