use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstanceDiscoveryCapacityLimit {
    #[serde(rename = "hard")]
    Hard,
    #[serde(rename = "recommended")]
    Recommended,
}

impl std::fmt::Display for InstanceDiscoveryCapacityLimit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Hard => write!(f, "hard"),
            Self::Recommended => write!(f, "recommended"),
        }
    }
}

impl Default for InstanceDiscoveryCapacityLimit {
    fn default() -> InstanceDiscoveryCapacityLimit {
        Self::Hard
    }
}
