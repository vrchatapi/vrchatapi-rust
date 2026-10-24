use crate::models;
use serde::{Deserialize, Serialize};

/// A ready-made request to an API route whose results the client shows. The fields depend on `kind`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum InfoPushDataContentSource {
    #[serde(rename = "calendarDiscover")]
    CalendarDiscover(models::InfoPushDataContentSourceCalendarDiscover),
    #[serde(rename = "instanceDiscover")]
    InstanceDiscover(models::InfoPushDataContentSourceInstanceDiscover),
    #[serde(untagged)]
    InfoPushDataContentSourceUnknown(models::InfoPushDataContentSourceUnknown),
}

impl Default for InfoPushDataContentSource {
    fn default() -> Self {
        Self::CalendarDiscover(Default::default())
    }
}

impl From<models::InfoPushDataContentSourceCalendarDiscover> for InfoPushDataContentSource {
    fn from(value: models::InfoPushDataContentSourceCalendarDiscover) -> Self {
        Self::CalendarDiscover(value)
    }
}

impl TryFrom<InfoPushDataContentSource> for models::InfoPushDataContentSourceCalendarDiscover {
    type Error = InfoPushDataContentSource;

    fn try_from(value: InfoPushDataContentSource) -> Result<Self, Self::Error> {
        match value {
            InfoPushDataContentSource::CalendarDiscover(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushDataContentSourceInstanceDiscover> for InfoPushDataContentSource {
    fn from(value: models::InfoPushDataContentSourceInstanceDiscover) -> Self {
        Self::InstanceDiscover(value)
    }
}

impl TryFrom<InfoPushDataContentSource> for models::InfoPushDataContentSourceInstanceDiscover {
    type Error = InfoPushDataContentSource;

    fn try_from(value: InfoPushDataContentSource) -> Result<Self, Self::Error> {
        match value {
            InfoPushDataContentSource::InstanceDiscover(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushDataContentSourceUnknown> for InfoPushDataContentSource {
    fn from(value: models::InfoPushDataContentSourceUnknown) -> Self {
        Self::InfoPushDataContentSourceUnknown(value)
    }
}

impl TryFrom<InfoPushDataContentSource> for models::InfoPushDataContentSourceUnknown {
    type Error = InfoPushDataContentSource;

    fn try_from(value: InfoPushDataContentSource) -> Result<Self, Self::Error> {
        match value {
            InfoPushDataContentSource::InfoPushDataContentSourceUnknown(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
