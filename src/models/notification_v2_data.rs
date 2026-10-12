use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NotificationV2Data {
    NotificationEmpty(serde_json::Value),
    NotificationV2DataBadgeEarned(models::NotificationV2DataBadgeEarned),
    NotificationV2DataBoop(models::NotificationV2DataBoop),
    NotificationV2DataEventAnnouncement(models::NotificationV2DataEventAnnouncement),
    NotificationV2DataGroupInformative(models::NotificationV2DataGroupInformative),
    NotificationV2DataGroupTransfer(models::NotificationV2DataGroupTransfer),
}

impl Default for NotificationV2Data {
    fn default() -> Self {
        Self::NotificationEmpty(Default::default())
    }
}

impl From<models::NotificationV2DataBadgeEarned> for NotificationV2Data {
    fn from(value: models::NotificationV2DataBadgeEarned) -> Self {
        Self::NotificationV2DataBadgeEarned(value)
    }
}

impl TryFrom<NotificationV2Data> for models::NotificationV2DataBadgeEarned {
    type Error = NotificationV2Data;

    fn try_from(value: NotificationV2Data) -> Result<Self, Self::Error> {
        match value {
            NotificationV2Data::NotificationV2DataBadgeEarned(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::NotificationV2DataBoop> for NotificationV2Data {
    fn from(value: models::NotificationV2DataBoop) -> Self {
        Self::NotificationV2DataBoop(value)
    }
}

impl TryFrom<NotificationV2Data> for models::NotificationV2DataBoop {
    type Error = NotificationV2Data;

    fn try_from(value: NotificationV2Data) -> Result<Self, Self::Error> {
        match value {
            NotificationV2Data::NotificationV2DataBoop(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::NotificationV2DataEventAnnouncement> for NotificationV2Data {
    fn from(value: models::NotificationV2DataEventAnnouncement) -> Self {
        Self::NotificationV2DataEventAnnouncement(value)
    }
}

impl TryFrom<NotificationV2Data> for models::NotificationV2DataEventAnnouncement {
    type Error = NotificationV2Data;

    fn try_from(value: NotificationV2Data) -> Result<Self, Self::Error> {
        match value {
            NotificationV2Data::NotificationV2DataEventAnnouncement(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::NotificationV2DataGroupInformative> for NotificationV2Data {
    fn from(value: models::NotificationV2DataGroupInformative) -> Self {
        Self::NotificationV2DataGroupInformative(value)
    }
}

impl TryFrom<NotificationV2Data> for models::NotificationV2DataGroupInformative {
    type Error = NotificationV2Data;

    fn try_from(value: NotificationV2Data) -> Result<Self, Self::Error> {
        match value {
            NotificationV2Data::NotificationV2DataGroupInformative(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::NotificationV2DataGroupTransfer> for NotificationV2Data {
    fn from(value: models::NotificationV2DataGroupTransfer) -> Self {
        Self::NotificationV2DataGroupTransfer(value)
    }
}

impl TryFrom<NotificationV2Data> for models::NotificationV2DataGroupTransfer {
    type Error = NotificationV2Data;

    fn try_from(value: NotificationV2Data) -> Result<Self, Self::Error> {
        match value {
            NotificationV2Data::NotificationV2DataGroupTransfer(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
