use crate::models;
use serde::{Deserialize, Serialize};

/// A notification delivered over the websocket. The shape of `details` depends on `type`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebsocketNotification {
    #[serde(rename = "boop")]
    Boop(models::WebsocketNotificationDetailBoop),
    #[serde(rename = "invite")]
    Invite(models::WebsocketNotificationDetailInvite),
    #[serde(rename = "invite-response")]
    InviteResponse(models::WebsocketNotificationDetailInviteResponse),
    #[serde(rename = "request-invite")]
    RequestInvite(models::WebsocketNotificationDetailRequestInvite),
    #[serde(rename = "request-invite-response")]
    RequestInviteResponse(models::WebsocketNotificationDetailRequestInviteResponse),
    #[serde(rename = "vote-to-kick")]
    VoteToKick(models::WebsocketNotificationDetailVoteToKick),
    #[serde(untagged)]
    WebsocketNotificationUnknown(models::WebsocketNotificationUnknown),
}

impl Default for WebsocketNotification {
    fn default() -> Self {
        Self::Boop(Default::default())
    }
}
