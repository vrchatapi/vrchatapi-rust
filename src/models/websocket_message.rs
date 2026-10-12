use crate::models;
use serde::{Deserialize, Serialize};

/// A message received over the websocket. The shape of `content` depends on `type`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebsocketMessage {
    #[serde(rename = "clear-notification")]
    ClearNotification(models::WebsocketClearNotification),
    #[serde(rename = "content-refresh")]
    ContentRefresh(models::WebsocketContentRefreshEncoded),
    #[serde(rename = "friend-active")]
    FriendActive(models::WebsocketFriendActiveEncoded),
    #[serde(rename = "friend-add")]
    FriendAdd(models::WebsocketFriendAddEncoded),
    #[serde(rename = "friend-delete")]
    FriendDelete(models::WebsocketFriendDeleteEncoded),
    #[serde(rename = "friend-location")]
    FriendLocation(models::WebsocketFriendLocationEncoded),
    #[serde(rename = "friend-offline")]
    FriendOffline(models::WebsocketFriendOfflineEncoded),
    #[serde(rename = "friend-online")]
    FriendOnline(models::WebsocketFriendOnlineEncoded),
    #[serde(rename = "friend-update")]
    FriendUpdate(models::WebsocketFriendUpdateEncoded),
    #[serde(rename = "group-joined")]
    GroupJoined(models::WebsocketGroupJoinedEncoded),
    #[serde(rename = "group-left")]
    GroupLeft(models::WebsocketGroupLeftEncoded),
    #[serde(rename = "group-member-updated")]
    GroupMemberUpdated(models::WebsocketGroupMemberUpdatedEncoded),
    #[serde(rename = "group-role-updated")]
    GroupRoleUpdated(models::WebsocketGroupRoleUpdatedEncoded),
    #[serde(rename = "hide-notification")]
    HideNotification(models::WebsocketHideNotification),
    #[serde(rename = "instance-queue-joined")]
    InstanceQueueJoined(models::WebsocketInstanceQueueJoinedEncoded),
    #[serde(rename = "instance-queue-ready")]
    InstanceQueueReady(models::WebsocketInstanceQueueReadyEncoded),
    #[serde(rename = "notification")]
    Notification(models::WebsocketNotificationEncoded),
    #[serde(rename = "notification-v2")]
    NotificationV2(models::WebsocketNotificationV2Encoded),
    #[serde(rename = "notification-v2-delete")]
    NotificationV2Delete(models::WebsocketNotificationV2DeleteEncoded),
    #[serde(rename = "notification-v2-update")]
    NotificationV2Update(models::WebsocketNotificationV2UpdateEncoded),
    #[serde(rename = "response-notification")]
    ResponseNotification(models::WebsocketResponseNotificationEncoded),
    #[serde(rename = "see-notification")]
    SeeNotification(models::WebsocketSeeNotification),
    #[serde(rename = "user-badge-assigned")]
    UserBadgeAssigned(models::WebsocketUserBadgeAssignedEncoded),
    #[serde(rename = "user-badge-unassigned")]
    UserBadgeUnassigned(models::WebsocketUserBadgeUnassignedEncoded),
    #[serde(rename = "user-location")]
    UserLocation(models::WebsocketUserLocationEncoded),
    #[serde(rename = "user-update")]
    UserUpdate(models::WebsocketUserUpdateEncoded),
    #[serde(untagged)]
    WebsocketMessageUnknown(models::WebsocketMessageUnknown),
}

impl Default for WebsocketMessage {
    fn default() -> Self {
        Self::ClearNotification(Default::default())
    }
}

impl From<models::WebsocketClearNotification> for WebsocketMessage {
    fn from(value: models::WebsocketClearNotification) -> Self {
        Self::ClearNotification(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketClearNotification {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::ClearNotification(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketContentRefreshEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketContentRefreshEncoded) -> Self {
        Self::ContentRefresh(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketContentRefreshEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::ContentRefresh(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketFriendActiveEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketFriendActiveEncoded) -> Self {
        Self::FriendActive(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketFriendActiveEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::FriendActive(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketFriendAddEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketFriendAddEncoded) -> Self {
        Self::FriendAdd(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketFriendAddEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::FriendAdd(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketFriendDeleteEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketFriendDeleteEncoded) -> Self {
        Self::FriendDelete(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketFriendDeleteEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::FriendDelete(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketFriendLocationEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketFriendLocationEncoded) -> Self {
        Self::FriendLocation(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketFriendLocationEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::FriendLocation(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketFriendOfflineEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketFriendOfflineEncoded) -> Self {
        Self::FriendOffline(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketFriendOfflineEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::FriendOffline(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketFriendOnlineEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketFriendOnlineEncoded) -> Self {
        Self::FriendOnline(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketFriendOnlineEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::FriendOnline(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketFriendUpdateEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketFriendUpdateEncoded) -> Self {
        Self::FriendUpdate(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketFriendUpdateEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::FriendUpdate(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketGroupJoinedEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketGroupJoinedEncoded) -> Self {
        Self::GroupJoined(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketGroupJoinedEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::GroupJoined(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketGroupLeftEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketGroupLeftEncoded) -> Self {
        Self::GroupLeft(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketGroupLeftEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::GroupLeft(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketGroupMemberUpdatedEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketGroupMemberUpdatedEncoded) -> Self {
        Self::GroupMemberUpdated(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketGroupMemberUpdatedEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::GroupMemberUpdated(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketGroupRoleUpdatedEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketGroupRoleUpdatedEncoded) -> Self {
        Self::GroupRoleUpdated(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketGroupRoleUpdatedEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::GroupRoleUpdated(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketHideNotification> for WebsocketMessage {
    fn from(value: models::WebsocketHideNotification) -> Self {
        Self::HideNotification(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketHideNotification {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::HideNotification(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketInstanceQueueJoinedEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketInstanceQueueJoinedEncoded) -> Self {
        Self::InstanceQueueJoined(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketInstanceQueueJoinedEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::InstanceQueueJoined(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketInstanceQueueReadyEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketInstanceQueueReadyEncoded) -> Self {
        Self::InstanceQueueReady(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketInstanceQueueReadyEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::InstanceQueueReady(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketNotificationEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketNotificationEncoded) -> Self {
        Self::Notification(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketNotificationEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::Notification(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketNotificationV2Encoded> for WebsocketMessage {
    fn from(value: models::WebsocketNotificationV2Encoded) -> Self {
        Self::NotificationV2(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketNotificationV2Encoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::NotificationV2(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketNotificationV2DeleteEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketNotificationV2DeleteEncoded) -> Self {
        Self::NotificationV2Delete(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketNotificationV2DeleteEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::NotificationV2Delete(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketNotificationV2UpdateEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketNotificationV2UpdateEncoded) -> Self {
        Self::NotificationV2Update(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketNotificationV2UpdateEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::NotificationV2Update(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketResponseNotificationEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketResponseNotificationEncoded) -> Self {
        Self::ResponseNotification(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketResponseNotificationEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::ResponseNotification(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketSeeNotification> for WebsocketMessage {
    fn from(value: models::WebsocketSeeNotification) -> Self {
        Self::SeeNotification(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketSeeNotification {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::SeeNotification(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketUserBadgeAssignedEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketUserBadgeAssignedEncoded) -> Self {
        Self::UserBadgeAssigned(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketUserBadgeAssignedEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::UserBadgeAssigned(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketUserBadgeUnassignedEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketUserBadgeUnassignedEncoded) -> Self {
        Self::UserBadgeUnassigned(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketUserBadgeUnassignedEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::UserBadgeUnassigned(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketUserLocationEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketUserLocationEncoded) -> Self {
        Self::UserLocation(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketUserLocationEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::UserLocation(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketUserUpdateEncoded> for WebsocketMessage {
    fn from(value: models::WebsocketUserUpdateEncoded) -> Self {
        Self::UserUpdate(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketUserUpdateEncoded {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::UserUpdate(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::WebsocketMessageUnknown> for WebsocketMessage {
    fn from(value: models::WebsocketMessageUnknown) -> Self {
        Self::WebsocketMessageUnknown(value)
    }
}

impl TryFrom<WebsocketMessage> for models::WebsocketMessageUnknown {
    type Error = WebsocketMessage;

    fn try_from(value: WebsocketMessage) -> Result<Self, Self::Error> {
        match value {
            WebsocketMessage::WebsocketMessageUnknown(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
