use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupCalendarEventDelete {
    #[serde(flatten)]
    pub all_of_0: models::GroupAuditLogEntryDataGroupCalendarEventCreate,
    #[serde(flatten)]
    pub GroupAuditLogEntryDataGroupCalendarEventDelete: serde_json::Value,
}
