use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserNoteTargetUser {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl UserNoteTargetUser {
    pub fn new() -> UserNoteTargetUser {
        UserNoteTargetUser {
            id: None,
            display_name: None,
        }
    }
}
