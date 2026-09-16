use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InventoryItemType {
    #[serde(rename = "bundle")]
    Bundle,
    #[serde(rename = "droneskin")]
    Droneskin,
    #[serde(rename = "emoji")]
    Emoji,
    #[serde(rename = "iconFrame")]
    IconFrame,
    #[serde(rename = "nameplateEffect")]
    NameplateEffect,
    #[serde(rename = "portalskin")]
    Portalskin,
    #[serde(rename = "profileEffect")]
    ProfileEffect,
    #[serde(rename = "prop")]
    Prop,
    #[serde(rename = "sticker")]
    Sticker,
    #[serde(rename = "warpeffect")]
    Warpeffect,
}

impl std::fmt::Display for InventoryItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Bundle => write!(f, "bundle"),
            Self::Droneskin => write!(f, "droneskin"),
            Self::Emoji => write!(f, "emoji"),
            Self::IconFrame => write!(f, "iconFrame"),
            Self::NameplateEffect => write!(f, "nameplateEffect"),
            Self::Portalskin => write!(f, "portalskin"),
            Self::ProfileEffect => write!(f, "profileEffect"),
            Self::Prop => write!(f, "prop"),
            Self::Sticker => write!(f, "sticker"),
            Self::Warpeffect => write!(f, "warpeffect"),
        }
    }
}

impl Default for InventoryItemType {
    fn default() -> InventoryItemType {
        Self::Bundle
    }
}
