use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FrontendVariant {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "public")]
    Public,
}

impl std::fmt::Display for FrontendVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Internal => write!(f, "internal"),
            Self::Public => write!(f, "public"),
        }
    }
}

impl Default for FrontendVariant {
    fn default() -> FrontendVariant {
        Self::Internal
    }
}
