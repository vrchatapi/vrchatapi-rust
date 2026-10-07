use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupPostCreate {
    #[serde(flatten)]
    pub all_of_0: models::GroupAuditLogEntryDataGroupPost,
    #[serde(flatten)]
    pub GroupAuditLogEntryDataGroupPostCreate: serde_json::Value,
}
