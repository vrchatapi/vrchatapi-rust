use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeclineGroupInviteRequest {
    #[serde(rename = "block", skip_serializing_if = "Option::is_none")]
    pub block: Option<bool>,
}

impl DeclineGroupInviteRequest {
    pub fn new() -> DeclineGroupInviteRequest {
        DeclineGroupInviteRequest { block: None }
    }
}
