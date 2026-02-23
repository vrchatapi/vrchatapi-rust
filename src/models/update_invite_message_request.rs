use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateInviteMessageRequest {
    #[serde(rename = "message")]
    pub message: String,
}

impl UpdateInviteMessageRequest {
    pub fn new(message: String) -> UpdateInviteMessageRequest {
        UpdateInviteMessageRequest { message }
    }
}
