use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PublicProfileThemeId {
    AnyOf0(String),
    AnyOf1(String),
}

impl Default for PublicProfileThemeId {
    fn default() -> Self {
        Self::AnyOf0(Default::default())
    }
}
