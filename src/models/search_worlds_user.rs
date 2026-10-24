use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SearchWorldsUser {
    #[serde(rename = "friends")]
    Friends,
    #[serde(rename = "me")]
    Me,
}

impl std::fmt::Display for SearchWorldsUser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Friends => write!(f, "friends"),
            Self::Me => write!(f, "me"),
        }
    }
}

impl Default for SearchWorldsUser {
    fn default() -> SearchWorldsUser {
        Self::Friends
    }
}
