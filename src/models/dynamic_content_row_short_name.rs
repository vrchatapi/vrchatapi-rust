use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DynamicContentRowShortName {
    String(String),
    LocalizedString(models::LocalizedString),
}

impl Default for DynamicContentRowShortName {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
