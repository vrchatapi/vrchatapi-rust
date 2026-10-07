use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupInstanceModeration {
    /// The instance the target was kicked from or warned in.
    #[serde(rename = "location")]
    pub location: String,
}

impl GroupAuditLogEntryDataGroupInstanceModeration {
    pub fn new(location: String) -> GroupAuditLogEntryDataGroupInstanceModeration {
        GroupAuditLogEntryDataGroupInstanceModeration { location }
    }
}
