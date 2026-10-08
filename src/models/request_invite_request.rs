use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestInviteRequest {
    #[serde(rename = "requestSlot", skip_serializing_if = "Option::is_none")]
    pub request_slot: Option<i32>,
}

impl RequestInviteRequest {
    pub fn new() -> RequestInviteRequest {
        RequestInviteRequest { request_slot: None }
    }
}
