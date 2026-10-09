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
