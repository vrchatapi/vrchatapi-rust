use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupRoleDelete {
    #[serde(flatten)]
    pub all_of_0: models::GroupAuditLogEntryDataGroupRole,
    #[serde(flatten)]
    pub GroupAuditLogEntryDataGroupRoleDelete: serde_json::Value,
}
