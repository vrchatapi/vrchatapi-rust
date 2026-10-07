use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryUnknown {
    #[serde(flatten)]
    pub all_of_0: models::GroupAuditLogEntryBase,
    #[serde(flatten)]
    pub GroupAuditLogEntryUnknown: serde_json::Value,
}
