use crate::models;
use serde::{Deserialize, Serialize};

/// A row of content. The fields depend on `type`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DynamicContentRow {
    #[serde(rename = "avatar")]
    Avatar(models::DynamicContentRowAvatars),
    #[serde(rename = "events")]
    Events(models::DynamicContentRowEvents),
    #[serde(rename = "instances")]
    Instances(models::DynamicContentRowInstances),
    #[serde(untagged)]
    DynamicContentRowWorlds(models::DynamicContentRowWorlds),
}

impl Default for DynamicContentRow {
    fn default() -> Self {
        Self::Avatar(Default::default())
    }
}

impl From<models::DynamicContentRowAvatars> for DynamicContentRow {
    fn from(value: models::DynamicContentRowAvatars) -> Self {
        Self::Avatar(value)
    }
}

impl TryFrom<DynamicContentRow> for models::DynamicContentRowAvatars {
    type Error = DynamicContentRow;

    fn try_from(value: DynamicContentRow) -> Result<Self, Self::Error> {
        match value {
            DynamicContentRow::Avatar(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::DynamicContentRowEvents> for DynamicContentRow {
    fn from(value: models::DynamicContentRowEvents) -> Self {
        Self::Events(value)
    }
}

impl TryFrom<DynamicContentRow> for models::DynamicContentRowEvents {
    type Error = DynamicContentRow;

    fn try_from(value: DynamicContentRow) -> Result<Self, Self::Error> {
        match value {
            DynamicContentRow::Events(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::DynamicContentRowInstances> for DynamicContentRow {
    fn from(value: models::DynamicContentRowInstances) -> Self {
        Self::Instances(value)
    }
}

impl TryFrom<DynamicContentRow> for models::DynamicContentRowInstances {
    type Error = DynamicContentRow;

    fn try_from(value: DynamicContentRow) -> Result<Self, Self::Error> {
        match value {
            DynamicContentRow::Instances(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::DynamicContentRowWorlds> for DynamicContentRow {
    fn from(value: models::DynamicContentRowWorlds) -> Self {
        Self::DynamicContentRowWorlds(value)
    }
}

impl TryFrom<DynamicContentRow> for models::DynamicContentRowWorlds {
    type Error = DynamicContentRow;

    fn try_from(value: DynamicContentRow) -> Result<Self, Self::Error> {
        match value {
            DynamicContentRow::DynamicContentRowWorlds(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
