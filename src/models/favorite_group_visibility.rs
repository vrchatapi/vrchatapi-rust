use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FavoriteGroupVisibility {
    #[serde(rename = "friends")]
    Friends,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
}

impl std::fmt::Display for FavoriteGroupVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Friends => write!(f, "friends"),
            Self::Private => write!(f, "private"),
            Self::Public => write!(f, "public"),
        }
    }
}

impl Default for FavoriteGroupVisibility {
    fn default() -> FavoriteGroupVisibility {
        Self::Friends
    }
}
