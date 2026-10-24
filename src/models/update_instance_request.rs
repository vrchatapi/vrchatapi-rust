use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateInstanceRequest {
    /// Calendar event to link to the instance. Send null to remove the current link.
    #[serde(
        rename = "calendarEntryId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub calendar_entry_id: Option<Option<String>>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl UpdateInstanceRequest {
    pub fn new() -> UpdateInstanceRequest {
        UpdateInstanceRequest {
            calendar_entry_id: None,
            display_name: None,
        }
    }
}
