use crate::models;
use serde::{Deserialize, Serialize};

/// The results of a content shelf's `contentSource`, as its `responseType` names them.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "responseType")]
pub enum InfoPushContent {
    #[serde(rename = "instance")]
    Instance(models::InfoPushContentInstances),
    #[serde(rename = "calendarEntry")]
    CalendarEntry(models::InfoPushContentCalendarEntries),
    #[serde(untagged)]
    InfoPushContentUnknown(models::InfoPushContentUnknown),
}

impl Default for InfoPushContent {
    fn default() -> Self {
        Self::Instance(Default::default())
    }
}

impl From<models::InfoPushContentInstances> for InfoPushContent {
    fn from(value: models::InfoPushContentInstances) -> Self {
        Self::Instance(value)
    }
}

impl TryFrom<InfoPushContent> for models::InfoPushContentInstances {
    type Error = InfoPushContent;

    fn try_from(value: InfoPushContent) -> Result<Self, Self::Error> {
        match value {
            InfoPushContent::Instance(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushContentCalendarEntries> for InfoPushContent {
    fn from(value: models::InfoPushContentCalendarEntries) -> Self {
        Self::CalendarEntry(value)
    }
}

impl TryFrom<InfoPushContent> for models::InfoPushContentCalendarEntries {
    type Error = InfoPushContent;

    fn try_from(value: InfoPushContent) -> Result<Self, Self::Error> {
        match value {
            InfoPushContent::CalendarEntry(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushContentUnknown> for InfoPushContent {
    fn from(value: models::InfoPushContentUnknown) -> Self {
        Self::InfoPushContentUnknown(value)
    }
}

impl TryFrom<InfoPushContent> for models::InfoPushContentUnknown {
    type Error = InfoPushContent;

    fn try_from(value: InfoPushContent) -> Result<Self, Self::Error> {
        match value {
            InfoPushContent::InfoPushContentUnknown(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
