use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SortOptionProductPurchase {
    #[serde(rename = "expireDate")]
    ExpireDate,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "purchaseDate")]
    PurchaseDate,
}

impl std::fmt::Display for SortOptionProductPurchase {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ExpireDate => write!(f, "expireDate"),
            Self::Name => write!(f, "name"),
            Self::PurchaseDate => write!(f, "purchaseDate"),
        }
    }
}

impl Default for SortOptionProductPurchase {
    fn default() -> SortOptionProductPurchase {
        Self::ExpireDate
    }
}
