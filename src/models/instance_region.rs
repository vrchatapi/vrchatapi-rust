use crate::models;
use serde::{Deserialize, Serialize};

/// Instance region
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstanceRegion {
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "jp")]
    Jp,
    #[serde(rename = "us")]
    Us,
    #[serde(rename = "use")]
    Use,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for InstanceRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Eu => write!(f, "eu"),
            Self::Jp => write!(f, "jp"),
            Self::Us => write!(f, "us"),
            Self::Use => write!(f, "use"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

impl Default for InstanceRegion {
    fn default() -> InstanceRegion {
        Self::Eu
    }
}
