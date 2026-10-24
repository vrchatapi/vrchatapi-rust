use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CosmeticItemType {
    #[serde(rename = "droneskin")]
    Droneskin,
    #[serde(rename = "iconFrame")]
    IconFrame,
    #[serde(rename = "nameplateEffect")]
    NameplateEffect,
    #[serde(rename = "portalskin")]
    Portalskin,
    #[serde(rename = "profileBackground")]
    ProfileBackground,
    #[serde(rename = "profileEffect")]
    ProfileEffect,
    #[serde(rename = "warpeffect")]
    Warpeffect,
}

impl std::fmt::Display for CosmeticItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Droneskin => write!(f, "droneskin"),
            Self::IconFrame => write!(f, "iconFrame"),
            Self::NameplateEffect => write!(f, "nameplateEffect"),
            Self::Portalskin => write!(f, "portalskin"),
            Self::ProfileBackground => write!(f, "profileBackground"),
            Self::ProfileEffect => write!(f, "profileEffect"),
            Self::Warpeffect => write!(f, "warpeffect"),
        }
    }
}

impl Default for CosmeticItemType {
    fn default() -> CosmeticItemType {
        Self::Droneskin
    }
}
