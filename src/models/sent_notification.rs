use crate::models;
use serde::{Deserialize, Serialize};

/// A notification as its sender receives it back. The shape of `details` depends on `type`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SentNotification {
    #[serde(rename = "boop")]
    Boop(models::SentNotificationBoop),
    #[serde(rename = "friendRequest")]
    FriendRequest(models::SentNotificationFriendRequest),
    #[serde(rename = "invite")]
    Invite(models::SentNotificationInvite),
    #[serde(rename = "inviteResponse")]
    InviteResponse(models::SentNotificationInviteResponse),
    #[serde(rename = "message")]
    Message(models::SentNotificationMessage),
    #[serde(rename = "requestInvite")]
    RequestInvite(models::SentNotificationRequestInvite),
    #[serde(rename = "requestInviteResponse")]
    RequestInviteResponse(models::SentNotificationRequestInviteResponse),
    #[serde(rename = "votetokick")]
    Votetokick(models::SentNotificationVoteToKick),
    #[serde(untagged)]
    SentNotificationUnknown(models::SentNotificationUnknown),
}

impl Default for SentNotification {
    fn default() -> Self {
        Self::Boop(Default::default())
    }
}

impl From<models::SentNotificationBoop> for SentNotification {
    fn from(value: models::SentNotificationBoop) -> Self {
        Self::Boop(value)
    }
}

impl TryFrom<SentNotification> for models::SentNotificationBoop {
    type Error = SentNotification;

    fn try_from(value: SentNotification) -> Result<Self, Self::Error> {
        match value {
            SentNotification::Boop(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::SentNotificationFriendRequest> for SentNotification {
    fn from(value: models::SentNotificationFriendRequest) -> Self {
        Self::FriendRequest(value)
    }
}

impl TryFrom<SentNotification> for models::SentNotificationFriendRequest {
    type Error = SentNotification;

    fn try_from(value: SentNotification) -> Result<Self, Self::Error> {
        match value {
            SentNotification::FriendRequest(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::SentNotificationInvite> for SentNotification {
    fn from(value: models::SentNotificationInvite) -> Self {
        Self::Invite(value)
    }
}

impl TryFrom<SentNotification> for models::SentNotificationInvite {
    type Error = SentNotification;

    fn try_from(value: SentNotification) -> Result<Self, Self::Error> {
        match value {
            SentNotification::Invite(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::SentNotificationInviteResponse> for SentNotification {
    fn from(value: models::SentNotificationInviteResponse) -> Self {
        Self::InviteResponse(value)
    }
}

impl TryFrom<SentNotification> for models::SentNotificationInviteResponse {
    type Error = SentNotification;

    fn try_from(value: SentNotification) -> Result<Self, Self::Error> {
        match value {
            SentNotification::InviteResponse(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::SentNotificationMessage> for SentNotification {
    fn from(value: models::SentNotificationMessage) -> Self {
        Self::Message(value)
    }
}

impl TryFrom<SentNotification> for models::SentNotificationMessage {
    type Error = SentNotification;

    fn try_from(value: SentNotification) -> Result<Self, Self::Error> {
        match value {
            SentNotification::Message(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::SentNotificationRequestInvite> for SentNotification {
    fn from(value: models::SentNotificationRequestInvite) -> Self {
        Self::RequestInvite(value)
    }
}

impl TryFrom<SentNotification> for models::SentNotificationRequestInvite {
    type Error = SentNotification;

    fn try_from(value: SentNotification) -> Result<Self, Self::Error> {
        match value {
            SentNotification::RequestInvite(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::SentNotificationRequestInviteResponse> for SentNotification {
    fn from(value: models::SentNotificationRequestInviteResponse) -> Self {
        Self::RequestInviteResponse(value)
    }
}

impl TryFrom<SentNotification> for models::SentNotificationRequestInviteResponse {
    type Error = SentNotification;

    fn try_from(value: SentNotification) -> Result<Self, Self::Error> {
        match value {
            SentNotification::RequestInviteResponse(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::SentNotificationVoteToKick> for SentNotification {
    fn from(value: models::SentNotificationVoteToKick) -> Self {
        Self::Votetokick(value)
    }
}

impl TryFrom<SentNotification> for models::SentNotificationVoteToKick {
    type Error = SentNotification;

    fn try_from(value: SentNotification) -> Result<Self, Self::Error> {
        match value {
            SentNotification::Votetokick(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::SentNotificationUnknown> for SentNotification {
    fn from(value: models::SentNotificationUnknown) -> Self {
        Self::SentNotificationUnknown(value)
    }
}

impl TryFrom<SentNotification> for models::SentNotificationUnknown {
    type Error = SentNotification;

    fn try_from(value: SentNotification) -> Result<Self, Self::Error> {
        match value {
            SentNotification::SentNotificationUnknown(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
