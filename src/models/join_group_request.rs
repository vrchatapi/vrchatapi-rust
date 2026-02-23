use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JoinGroupRequest {
    #[serde(rename = "inviteId", skip_serializing_if = "Option::is_none")]
    pub invite_id: Option<String>,
}

impl JoinGroupRequest {
    pub fn new() -> JoinGroupRequest {
        JoinGroupRequest { invite_id: None }
    }
}
