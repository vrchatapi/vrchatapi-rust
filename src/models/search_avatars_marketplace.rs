use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SearchAvatarsMarketplace {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "free")]
    Free,
    #[serde(rename = "paid")]
    Paid,
}

impl std::fmt::Display for SearchAvatarsMarketplace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Free => write!(f, "free"),
            Self::Paid => write!(f, "paid"),
        }
    }
}

impl Default for SearchAvatarsMarketplace {
    fn default() -> SearchAvatarsMarketplace {
        Self::All
    }
}
