use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstanceDiscoveryMode {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "population")]
    Population,
    #[serde(rename = "random")]
    Random,
}

impl std::fmt::Display for InstanceDiscoveryMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Match => write!(f, "match"),
            Self::Population => write!(f, "population"),
            Self::Random => write!(f, "random"),
        }
    }
}

impl Default for InstanceDiscoveryMode {
    fn default() -> InstanceDiscoveryMode {
        Self::Active
    }
}
