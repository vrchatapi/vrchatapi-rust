use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InventoryFlag {
    #[serde(rename = "archivable")]
    Archivable,
    #[serde(rename = "cloneable")]
    Cloneable,
    #[serde(rename = "consumable")]
    Consumable,
    #[serde(rename = "equippable")]
    Equippable,
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "global_visible")]
    GlobalVisible,
    #[serde(rename = "instantiatable")]
    Instantiatable,
    #[serde(rename = "trashable")]
    Trashable,
    #[serde(rename = "ugc")]
    Ugc,
    #[serde(rename = "unique")]
    Unique,
    #[serde(rename = "vrc_plus_exclusive")]
    VrcPlusExclusive,
}

impl std::fmt::Display for InventoryFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Archivable => write!(f, "archivable"),
            Self::Cloneable => write!(f, "cloneable"),
            Self::Consumable => write!(f, "consumable"),
            Self::Equippable => write!(f, "equippable"),
            Self::Global => write!(f, "global"),
            Self::GlobalVisible => write!(f, "global_visible"),
            Self::Instantiatable => write!(f, "instantiatable"),
            Self::Trashable => write!(f, "trashable"),
            Self::Ugc => write!(f, "ugc"),
            Self::Unique => write!(f, "unique"),
            Self::VrcPlusExclusive => write!(f, "vrc_plus_exclusive"),
        }
    }
}

impl Default for InventoryFlag {
    fn default() -> InventoryFlag {
        Self::Archivable
    }
}
