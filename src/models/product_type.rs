use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProductType {
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
    #[serde(rename = "role")]
    Role,
}

impl std::fmt::Display for ProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Avatar => write!(f, "avatar"),
            Self::Credit => write!(f, "credit"),
            Self::Inventory => write!(f, "inventory"),
            Self::Listing => write!(f, "listing"),
            Self::TestBirdy => write!(f, "test_birdy"),
            Self::Udon => write!(f, "udon"),
            Self::Role => write!(f, "role"),
        }
    }
}

impl Default for ProductType {
    fn default() -> ProductType {
        Self::Avatar
    }
}
