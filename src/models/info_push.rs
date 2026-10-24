use crate::models;
use serde::{Deserialize, Serialize};

/// An item VRChat pushes to the client. The shape of `data` depends on `type`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InfoPush {
    #[serde(rename = "ContentShelf")]
    ContentShelf(models::InfoPushContentShelf),
    #[serde(untagged)]
    InfoPushUntyped(models::InfoPushUntyped),
}

impl Default for InfoPush {
    fn default() -> Self {
        Self::ContentShelf(Default::default())
    }
}

impl From<models::InfoPushContentShelf> for InfoPush {
    fn from(value: models::InfoPushContentShelf) -> Self {
        Self::ContentShelf(value)
    }
}

impl TryFrom<InfoPush> for models::InfoPushContentShelf {
    type Error = InfoPush;

    fn try_from(value: InfoPush) -> Result<Self, Self::Error> {
        match value {
            InfoPush::ContentShelf(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::InfoPushUntyped> for InfoPush {
    fn from(value: models::InfoPushUntyped) -> Self {
        Self::InfoPushUntyped(value)
    }
}

impl TryFrom<InfoPush> for models::InfoPushUntyped {
    type Error = InfoPush;

    fn try_from(value: InfoPush) -> Result<Self, Self::Error> {
        match value {
            InfoPush::InfoPushUntyped(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
