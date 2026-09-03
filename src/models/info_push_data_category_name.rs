use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InfoPushDataCategoryName {
    String(String),
    LocalizedString(models::LocalizedString),
}

impl Default for InfoPushDataCategoryName {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
