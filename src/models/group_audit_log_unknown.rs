use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogUnknown {
    #[serde(flatten)]
    pub group_audit_log_base: models::GroupAuditLogBase,
    #[serde(rename = "data")]
    pub data: std::collections::HashMap<String, serde_json::Value>,
    /// The type of event that occurred.
    #[serde(rename = "eventType")]
    pub event_type: String,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

impl GroupAuditLogUnknown {
    pub fn new(
        group_audit_log_base: models::GroupAuditLogBase,
        data: std::collections::HashMap<String, serde_json::Value>,
        event_type: String,
        target_id: String,
    ) -> GroupAuditLogUnknown {
        GroupAuditLogUnknown {
            group_audit_log_base,
            data,
            event_type,
            target_id,
        }
    }
}
