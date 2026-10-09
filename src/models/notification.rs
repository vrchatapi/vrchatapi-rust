use crate::models;
use serde::{Deserialize, Serialize};

/// A notification. The shape of `details` depends on `type`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Notification {
    #[serde(rename = "boop")]
    Boop(models::NotificationBoop),
    #[serde(rename = "friendRequest")]
    FriendRequest(models::NotificationFriendRequest),
    #[serde(rename = "invite")]
    Invite(models::NotificationInvite),
    #[serde(rename = "inviteResponse")]
    InviteResponse(models::NotificationInviteResponse),
    #[serde(rename = "message")]
    Message(models::NotificationMessage),
    #[serde(rename = "requestInvite")]
    RequestInvite(models::NotificationRequestInvite),
    #[serde(rename = "requestInviteResponse")]
    RequestInviteResponse(models::NotificationRequestInviteResponse),
    #[serde(rename = "votetokick")]
    Votetokick(models::NotificationVoteToKick),
    #[serde(untagged)]
    NotificationUnknown(models::NotificationUnknown),
}

impl Default for Notification {
    fn default() -> Self {
        Self::Boop(Default::default())
    }
}
