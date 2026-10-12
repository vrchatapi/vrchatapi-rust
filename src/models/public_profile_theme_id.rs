use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PublicProfileThemeId {
    ProfileThemeID(String),
    AnyOf1(String),
}

impl Default for PublicProfileThemeId {
    fn default() -> Self {
        Self::ProfileThemeID(Default::default())
    }
}
