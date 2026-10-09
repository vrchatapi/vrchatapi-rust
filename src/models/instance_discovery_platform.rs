use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstanceDiscoveryPlatform {
    #[serde(rename = "android")]
    Android,
    #[serde(rename = "ios")]
    Ios,
    #[serde(rename = "standalonewindows")]
    Standalonewindows,
    #[serde(rename = "web")]
    Web,
}

impl std::fmt::Display for InstanceDiscoveryPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Android => write!(f, "android"),
            Self::Ios => write!(f, "ios"),
            Self::Standalonewindows => write!(f, "standalonewindows"),
            Self::Web => write!(f, "web"),
        }
    }
}

impl Default for InstanceDiscoveryPlatform {
    fn default() -> InstanceDiscoveryPlatform {
        Self::Android
    }
}
