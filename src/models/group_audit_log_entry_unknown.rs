use crate::models;
use serde::{Deserialize, Serialize};

/// GroupAuditLogEntryUnknown : An event whose `eventType` has no schema of its own.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryUnknown {
    #[serde(rename = "data")]
    pub data: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "eventType")]
    pub event_type: String,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogEntryUnknown {
    /// An event whose `eventType` has no schema of its own.
    pub fn new(
        data: std::collections::HashMap<String, serde_json::Value>,
        event_type: String,
        target_id: String,
    ) -> GroupAuditLogEntryUnknown {
        GroupAuditLogEntryUnknown {
            data,
            event_type,
            target_id,
        }
    }
}
