/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InviteRequest {
    /// InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance.
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    #[serde(rename = "messageSlot", skip_serializing_if = "Option::is_none")]
    pub message_slot: Option<i32>,
}

impl InviteRequest {
    pub fn new(instance_id: String) -> InviteRequest {
        InviteRequest {
            instance_id,
            message_slot: None,
        }
    }
}
