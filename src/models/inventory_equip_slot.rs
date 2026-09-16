use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InventoryEquipSlot {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "drone")]
    Drone,
    #[serde(rename = "iconFrame")]
    IconFrame,
    #[serde(rename = "nameplateEffect")]
    NameplateEffect,
    #[serde(rename = "portal")]
    Portal,
    #[serde(rename = "profileEffect")]
    ProfileEffect,
    #[serde(rename = "warp")]
    Warp,
}

impl std::fmt::Display for InventoryEquipSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, ""),
            Self::Drone => write!(f, "drone"),
            Self::IconFrame => write!(f, "iconFrame"),
            Self::NameplateEffect => write!(f, "nameplateEffect"),
            Self::Portal => write!(f, "portal"),
            Self::ProfileEffect => write!(f, "profileEffect"),
            Self::Warp => write!(f, "warp"),
        }
    }
}

impl Default for InventoryEquipSlot {
    fn default() -> InventoryEquipSlot {
        Self::Empty
    }
}
