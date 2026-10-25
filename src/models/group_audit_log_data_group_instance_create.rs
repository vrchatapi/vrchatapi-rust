use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogDataGroupInstanceCreate {
    /// The calendar entry ID if the instance was created from a calendar event.
    #[serde(rename = "calendarEntryId", deserialize_with = "Option::deserialize")]
    pub calendar_entry_id: Option<String>,
    #[serde(rename = "groupAccessType")]
    pub group_access_type: models::GroupAccessType,
    /// The role IDs that have access to the instance.
    #[serde(rename = "roleIds", deserialize_with = "Option::deserialize")]
    pub role_ids: Option<Vec<String>>,
}

impl GroupAuditLogDataGroupInstanceCreate {
    pub fn new(
        calendar_entry_id: Option<String>,
        group_access_type: models::GroupAccessType,
        role_ids: Option<Vec<String>>,
    ) -> GroupAuditLogDataGroupInstanceCreate {
        GroupAuditLogDataGroupInstanceCreate {
            calendar_entry_id,
            group_access_type,
            role_ids,
        }
    }
}
