use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupPostDelete {
    #[serde(flatten)]
    pub all_of_0: models::GroupAuditLogEntryDataGroupPost,
    #[serde(flatten)]
    pub GroupAuditLogEntryDataGroupPostDelete: serde_json::Value,
}
