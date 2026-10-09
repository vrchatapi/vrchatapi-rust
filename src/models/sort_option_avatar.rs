use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SortOptionAvatar {
    #[serde(rename = "_created_at")]
    CreatedAt,
    #[serde(rename = "_updated_at")]
    UpdatedAt,
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "countMonthlySales")]
    CountMonthlySales,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "exact")]
    Exact,
    #[serde(rename = "listingDate")]
    ListingDate,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "order")]
    Order,
    #[serde(rename = "performance")]
    Performance,
    #[serde(rename = "random")]
    Random,
    #[serde(rename = "relevance")]
    Relevance,
    #[serde(rename = "shuffle")]
    Shuffle,
    #[serde(rename = "trendRank")]
    TrendRank,
    #[serde(rename = "updated")]
    Updated,
}

impl std::fmt::Display for SortOptionAvatar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "_created_at"),
            Self::UpdatedAt => write!(f, "_updated_at"),
            Self::Contains => write!(f, "contains"),
            Self::CountMonthlySales => write!(f, "countMonthlySales"),
            Self::Created => write!(f, "created"),
            Self::Exact => write!(f, "exact"),
            Self::ListingDate => write!(f, "listingDate"),
            Self::Name => write!(f, "name"),
            Self::Order => write!(f, "order"),
            Self::Performance => write!(f, "performance"),
            Self::Random => write!(f, "random"),
            Self::Relevance => write!(f, "relevance"),
            Self::Shuffle => write!(f, "shuffle"),
            Self::TrendRank => write!(f, "trendRank"),
            Self::Updated => write!(f, "updated"),
        }
    }
}

impl Default for SortOptionAvatar {
    fn default() -> SortOptionAvatar {
        Self::CreatedAt
    }
}
