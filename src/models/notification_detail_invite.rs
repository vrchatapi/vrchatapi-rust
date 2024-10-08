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
pub struct NotificationDetailInvite {
    #[serde(rename = "inviteMessage", skip_serializing_if = "Option::is_none")]
    pub invite_message: Option<String>,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "worldId")]
    pub world_id: String,
    #[serde(rename = "worldName")]
    pub world_name: String,
}

impl NotificationDetailInvite {
    pub fn new(world_id: String, world_name: String) -> NotificationDetailInvite {
        NotificationDetailInvite {
            invite_message: None,
            world_id,
            world_name,
        }
    }
}
