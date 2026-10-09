use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreateInstanceRequestRegion {
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "jp")]
    Jp,
    #[serde(rename = "us")]
    Us,
    #[serde(rename = "use")]
    Use,
}

impl std::fmt::Display for CreateInstanceRequestRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Eu => write!(f, "eu"),
            Self::Jp => write!(f, "jp"),
            Self::Us => write!(f, "us"),
            Self::Use => write!(f, "use"),
        }
    }
}

impl Default for CreateInstanceRequestRegion {
    fn default() -> CreateInstanceRequestRegion {
        Self::Eu
    }
}
