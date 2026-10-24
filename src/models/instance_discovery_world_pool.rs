use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstanceDiscoveryWorldPool {
    #[serde(rename = "trending")]
    Trending,
}

impl std::fmt::Display for InstanceDiscoveryWorldPool {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Trending => write!(f, "trending"),
        }
    }
}

impl Default for InstanceDiscoveryWorldPool {
    fn default() -> InstanceDiscoveryWorldPool {
        Self::Trending
    }
}
