use crate::models;
use serde::{Deserialize, Serialize};

/// A notification. The shape of `data` depends on `type`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NotificationV2 {
    #[serde(rename = "group.announcement")]
    GroupAnnouncement(models::NotificationV2GroupAnnouncement),
    #[serde(rename = "group.invite")]
    GroupInvite(models::NotificationV2GroupInvite),
    #[serde(untagged)]
    NotificationV2Unknown(models::NotificationV2Unknown),
}

impl Default for NotificationV2 {
    fn default() -> Self {
        Self::GroupAnnouncement(Default::default())
    }
}

impl From<models::NotificationV2GroupAnnouncement> for NotificationV2 {
    fn from(value: models::NotificationV2GroupAnnouncement) -> Self {
        Self::GroupAnnouncement(value)
    }
}

impl TryFrom<NotificationV2> for models::NotificationV2GroupAnnouncement {
    type Error = NotificationV2;

    fn try_from(value: NotificationV2) -> Result<Self, Self::Error> {
        match value {
            NotificationV2::GroupAnnouncement(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::NotificationV2GroupInvite> for NotificationV2 {
    fn from(value: models::NotificationV2GroupInvite) -> Self {
        Self::GroupInvite(value)
    }
}

impl TryFrom<NotificationV2> for models::NotificationV2GroupInvite {
    type Error = NotificationV2;

    fn try_from(value: NotificationV2) -> Result<Self, Self::Error> {
        match value {
            NotificationV2::GroupInvite(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::NotificationV2Unknown> for NotificationV2 {
    fn from(value: models::NotificationV2Unknown) -> Self {
        Self::NotificationV2Unknown(value)
    }
}

impl TryFrom<NotificationV2> for models::NotificationV2Unknown {
    type Error = NotificationV2;

    fn try_from(value: NotificationV2) -> Result<Self, Self::Error> {
        match value {
            NotificationV2::NotificationV2Unknown(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
