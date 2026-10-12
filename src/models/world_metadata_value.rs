use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorldMetadataValue {
    WorldMetadataObject(std::collections::HashMap<String, serde_json::Value>),
    WorldMetadataArray(Vec<serde_json::Value>),
}

impl Default for WorldMetadataValue {
    fn default() -> Self {
        Self::WorldMetadataObject(Default::default())
    }
}
