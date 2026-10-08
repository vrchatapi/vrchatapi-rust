use crate::models;
use serde::{Deserialize, Serialize};

/// GroupAuditLogEntryUserIdChange : A user ID field's value before and after an update.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryUserIdChange {
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "new", deserialize_with = "Option::deserialize")]
    pub new: Option<String>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "old", deserialize_with = "Option::deserialize")]
    pub old: Option<String>,
}

impl GroupAuditLogEntryUserIdChange {
    /// A user ID field's value before and after an update.
    pub fn new(new: Option<String>, old: Option<String>) -> GroupAuditLogEntryUserIdChange {
        GroupAuditLogEntryUserIdChange { new, old }
    }
}
