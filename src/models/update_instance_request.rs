use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateInstanceRequest {
    /// Calendar event to link to the instance. Send null to remove the current link.
    #[serde(rename = "calendarEntryId", deserialize_with = "Option::deserialize")]
    pub calendar_entry_id: Option<String>,
}

impl UpdateInstanceRequest {
    pub fn new(calendar_entry_id: Option<String>) -> UpdateInstanceRequest {
        UpdateInstanceRequest { calendar_entry_id }
    }
}
