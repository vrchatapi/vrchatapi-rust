use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserSearchSort {
    #[serde(rename = "_created_at")]
    CreatedAt,
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "exact")]
    Exact,
    #[serde(rename = "last_login")]
    LastLogin,
    #[serde(rename = "magic")]
    Magic,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "nuisanceFactor")]
    NuisanceFactor,
    #[serde(rename = "relevance")]
    Relevance,
    #[serde(rename = "trust")]
    Trust,
}

impl std::fmt::Display for UserSearchSort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "_created_at"),
            Self::Contains => write!(f, "contains"),
            Self::Created => write!(f, "created"),
            Self::Exact => write!(f, "exact"),
            Self::LastLogin => write!(f, "last_login"),
            Self::Magic => write!(f, "magic"),
            Self::Name => write!(f, "name"),
            Self::NuisanceFactor => write!(f, "nuisanceFactor"),
            Self::Relevance => write!(f, "relevance"),
            Self::Trust => write!(f, "trust"),
        }
    }
}

impl Default for UserSearchSort {
    fn default() -> UserSearchSort {
        Self::CreatedAt
    }
}
