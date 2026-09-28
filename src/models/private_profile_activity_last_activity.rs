use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrivateProfileActivityLastActivity {
    AnyOf0(chrono::DateTime<chrono::FixedOffset>),
    AnyOf1(String),
}

impl Default for PrivateProfileActivityLastActivity {
    fn default() -> Self {
        Self::AnyOf0(Default::default())
    }
}
