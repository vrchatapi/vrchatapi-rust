use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupInstanceAnnouncement {
    /// The announcement message.
    #[serde(rename = "message")]
    pub message: String,
    /// The announcement title.
    #[serde(rename = "title")]
    pub title: String,
}

impl GroupAuditLogEntryDataGroupInstanceAnnouncement {
    pub fn new(message: String, title: String) -> GroupAuditLogEntryDataGroupInstanceAnnouncement {
        GroupAuditLogEntryDataGroupInstanceAnnouncement { message, title }
    }
}
