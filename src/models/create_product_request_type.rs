use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreateProductRequestType {
    #[serde(rename = "avatar")]
    Avatar,
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "inventory")]
    Inventory,
    #[serde(rename = "listing")]
    Listing,
    #[serde(rename = "test_birdy")]
    TestBirdy,
    #[serde(rename = "udon")]
    Udon,
}

impl std::fmt::Display for CreateProductRequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Avatar => write!(f, "avatar"),
            Self::Credit => write!(f, "credit"),
            Self::Inventory => write!(f, "inventory"),
            Self::Listing => write!(f, "listing"),
            Self::TestBirdy => write!(f, "test_birdy"),
            Self::Udon => write!(f, "udon"),
        }
    }
}

impl Default for CreateProductRequestType {
    fn default() -> CreateProductRequestType {
        Self::Avatar
    }
}
