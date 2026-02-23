use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InviteResponse {
    #[serde(rename = "responseSlot")]
    pub response_slot: i32,
}

impl InviteResponse {
    pub fn new(response_slot: i32) -> InviteResponse {
        InviteResponse { response_slot }
    }
}
