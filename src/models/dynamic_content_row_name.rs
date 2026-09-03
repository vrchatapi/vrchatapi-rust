use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DynamicContentRowName {
    String(String),
    LocalizedString(models::LocalizedString),
}

impl Default for DynamicContentRowName {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
