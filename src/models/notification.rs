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

impl From<models::NotificationBoop> for Notification {
    fn from(value: models::NotificationBoop) -> Self {
        Self::Boop(value)
    }
}

impl TryFrom<Notification> for models::NotificationBoop {
    type Error = Notification;

    fn try_from(value: Notification) -> Result<Self, Self::Error> {
        match value {
            Notification::Boop(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::NotificationFriendRequest> for Notification {
    fn from(value: models::NotificationFriendRequest) -> Self {
        Self::FriendRequest(value)
    }
}

impl TryFrom<Notification> for models::NotificationFriendRequest {
    type Error = Notification;

    fn try_from(value: Notification) -> Result<Self, Self::Error> {
        match value {
            Notification::FriendRequest(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::NotificationInvite> for Notification {
    fn from(value: models::NotificationInvite) -> Self {
        Self::Invite(value)
    }
}

impl TryFrom<Notification> for models::NotificationInvite {
    type Error = Notification;

    fn try_from(value: Notification) -> Result<Self, Self::Error> {
        match value {
            Notification::Invite(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::NotificationInviteResponse> for Notification {
    fn from(value: models::NotificationInviteResponse) -> Self {
        Self::InviteResponse(value)
    }
}

impl TryFrom<Notification> for models::NotificationInviteResponse {
    type Error = Notification;

    fn try_from(value: Notification) -> Result<Self, Self::Error> {
        match value {
            Notification::InviteResponse(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::NotificationMessage> for Notification {
    fn from(value: models::NotificationMessage) -> Self {
        Self::Message(value)
    }
}

impl TryFrom<Notification> for models::NotificationMessage {
    type Error = Notification;

    fn try_from(value: Notification) -> Result<Self, Self::Error> {
        match value {
            Notification::Message(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::NotificationRequestInvite> for Notification {
    fn from(value: models::NotificationRequestInvite) -> Self {
        Self::RequestInvite(value)
    }
}

impl TryFrom<Notification> for models::NotificationRequestInvite {
    type Error = Notification;

    fn try_from(value: Notification) -> Result<Self, Self::Error> {
        match value {
            Notification::RequestInvite(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::NotificationRequestInviteResponse> for Notification {
    fn from(value: models::NotificationRequestInviteResponse) -> Self {
        Self::RequestInviteResponse(value)
    }
}

impl TryFrom<Notification> for models::NotificationRequestInviteResponse {
    type Error = Notification;

    fn try_from(value: Notification) -> Result<Self, Self::Error> {
        match value {
            Notification::RequestInviteResponse(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::NotificationVoteToKick> for Notification {
    fn from(value: models::NotificationVoteToKick) -> Self {
        Self::Votetokick(value)
    }
}

impl TryFrom<Notification> for models::NotificationVoteToKick {
    type Error = Notification;

    fn try_from(value: Notification) -> Result<Self, Self::Error> {
        match value {
            Notification::Votetokick(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::NotificationUnknown> for Notification {
    fn from(value: models::NotificationUnknown) -> Self {
        Self::NotificationUnknown(value)
    }
}

impl TryFrom<Notification> for models::NotificationUnknown {
    type Error = Notification;

    fn try_from(value: Notification) -> Result<Self, Self::Error> {
        match value {
            Notification::NotificationUnknown(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
