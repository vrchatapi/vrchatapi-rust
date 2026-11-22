use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorOrEmpty {
    Color(String),
    AnyOf1(String),
}

impl Default for ColorOrEmpty {
    fn default() -> Self {
        Self::Color(Default::default())
    }
}
