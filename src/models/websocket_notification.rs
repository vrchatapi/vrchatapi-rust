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

impl From<models::WebsocketNotificationDetailBoop> for WebsocketNotification {
    fn from(value: models::WebsocketNotificationDetailBoop) -> Self {
        Self::Boop(value)
    }
}

impl TryFrom<WebsocketNotification> for models::WebsocketNotificationDetailBoop {
    type Error = WebsocketNotification;

    fn try_from(value: WebsocketNotification) -> Result<Self, Self::Error> {
        match value {
            WebsocketNotification::Boop(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketNotificationDetailInvite> for WebsocketNotification {
    fn from(value: models::WebsocketNotificationDetailInvite) -> Self {
        Self::Invite(value)
    }
}

impl TryFrom<WebsocketNotification> for models::WebsocketNotificationDetailInvite {
    type Error = WebsocketNotification;

    fn try_from(value: WebsocketNotification) -> Result<Self, Self::Error> {
        match value {
            WebsocketNotification::Invite(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketNotificationDetailInviteResponse> for WebsocketNotification {
    fn from(value: models::WebsocketNotificationDetailInviteResponse) -> Self {
        Self::InviteResponse(value)
    }
}

impl TryFrom<WebsocketNotification> for models::WebsocketNotificationDetailInviteResponse {
    type Error = WebsocketNotification;

    fn try_from(value: WebsocketNotification) -> Result<Self, Self::Error> {
        match value {
            WebsocketNotification::InviteResponse(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketNotificationDetailRequestInvite> for WebsocketNotification {
    fn from(value: models::WebsocketNotificationDetailRequestInvite) -> Self {
        Self::RequestInvite(value)
    }
}

impl TryFrom<WebsocketNotification> for models::WebsocketNotificationDetailRequestInvite {
    type Error = WebsocketNotification;

    fn try_from(value: WebsocketNotification) -> Result<Self, Self::Error> {
        match value {
            WebsocketNotification::RequestInvite(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketNotificationDetailRequestInviteResponse> for WebsocketNotification {
    fn from(value: models::WebsocketNotificationDetailRequestInviteResponse) -> Self {
        Self::RequestInviteResponse(value)
    }
}

impl TryFrom<WebsocketNotification> for models::WebsocketNotificationDetailRequestInviteResponse {
    type Error = WebsocketNotification;

    fn try_from(value: WebsocketNotification) -> Result<Self, Self::Error> {
        match value {
            WebsocketNotification::RequestInviteResponse(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketNotificationDetailVoteToKick> for WebsocketNotification {
    fn from(value: models::WebsocketNotificationDetailVoteToKick) -> Self {
        Self::VoteToKick(value)
    }
}

impl TryFrom<WebsocketNotification> for models::WebsocketNotificationDetailVoteToKick {
    type Error = WebsocketNotification;

    fn try_from(value: WebsocketNotification) -> Result<Self, Self::Error> {
        match value {
            WebsocketNotification::VoteToKick(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketNotificationUnknown> for WebsocketNotification {
    fn from(value: models::WebsocketNotificationUnknown) -> Self {
        Self::WebsocketNotificationUnknown(value)
    }
}

impl TryFrom<WebsocketNotification> for models::WebsocketNotificationUnknown {
    type Error = WebsocketNotification;

    fn try_from(value: WebsocketNotification) -> Result<Self, Self::Error> {
        match value {
            WebsocketNotification::WebsocketNotificationUnknown(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
