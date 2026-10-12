use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogDataGroupInstanceModeration {
    /// The instance the target was kicked from or warned in.
    #[serde(rename = "location")]
    pub location: String,
}

impl GroupAuditLogDataGroupInstanceModeration {
    pub fn new(location: String) -> GroupAuditLogDataGroupInstanceModeration {
        GroupAuditLogDataGroupInstanceModeration { location }
    }
}
