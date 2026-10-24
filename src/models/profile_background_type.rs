use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProfileBackgroundType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "gradient")]
    Gradient,
    #[serde(rename = "inventory")]
    Inventory,
    #[serde(rename = "texture")]
    Texture,
}

impl std::fmt::Display for ProfileBackgroundType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::Gradient => write!(f, "gradient"),
            Self::Inventory => write!(f, "inventory"),
            Self::Texture => write!(f, "texture"),
        }
    }
}

impl Default for ProfileBackgroundType {
    fn default() -> ProfileBackgroundType {
        Self::Default
    }
}
