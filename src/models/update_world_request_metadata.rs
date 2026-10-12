use crate::models;
use serde::{Deserialize, Serialize};

/// Replaces the world's metadata. A string is parsed as JSON.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWorldRequestMetadata {
    WorldMetadataValue(models::WorldMetadataValue),
    AnyOf1(String),
}

impl Default for UpdateWorldRequestMetadata {
    fn default() -> Self {
        Self::WorldMetadataValue(Default::default())
    }
}

impl From<models::WorldMetadataValue> for UpdateWorldRequestMetadata {
    fn from(value: models::WorldMetadataValue) -> Self {
        Self::WorldMetadataValue(value)
    }
}

impl TryFrom<UpdateWorldRequestMetadata> for models::WorldMetadataValue {
    type Error = UpdateWorldRequestMetadata;

    fn try_from(value: UpdateWorldRequestMetadata) -> Result<Self, Self::Error> {
        match value {
            UpdateWorldRequestMetadata::WorldMetadataValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
