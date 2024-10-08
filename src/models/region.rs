/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Region : API/Photon region.
/// API/Photon region.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Region {
    #[serde(rename = "us")]
    Us,
    #[serde(rename = "use")]
    Use,
    #[serde(rename = "usw")]
    Usw,
    #[serde(rename = "usx")]
    Usx,
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "jp")]
    Jp,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Us => write!(f, "us"),
            Self::Use => write!(f, "use"),
            Self::Usw => write!(f, "usw"),
            Self::Usx => write!(f, "usx"),
            Self::Eu => write!(f, "eu"),
            Self::Jp => write!(f, "jp"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

impl Default for Region {
    fn default() -> Region {
        Self::Us
    }
}
