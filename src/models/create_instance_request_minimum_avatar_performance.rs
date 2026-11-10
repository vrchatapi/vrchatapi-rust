use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreateInstanceRequestMinimumAvatarPerformance {
    #[serde(rename = "Good")]
    Good,
    #[serde(rename = "Medium")]
    Medium,
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Poor")]
    Poor,
}

impl std::fmt::Display for CreateInstanceRequestMinimumAvatarPerformance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Good => write!(f, "Good"),
            Self::Medium => write!(f, "Medium"),
            Self::None => write!(f, "None"),
            Self::Poor => write!(f, "Poor"),
        }
    }
}

impl Default for CreateInstanceRequestMinimumAvatarPerformance {
    fn default() -> CreateInstanceRequestMinimumAvatarPerformance {
        Self::Good
    }
}
