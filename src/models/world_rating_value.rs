use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WorldRatingValue {
    #[serde(rename = "dislike")]
    Dislike,
    #[serde(rename = "like")]
    Like,
}

impl std::fmt::Display for WorldRatingValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Dislike => write!(f, "dislike"),
            Self::Like => write!(f, "like"),
        }
    }
}

impl Default for WorldRatingValue {
    fn default() -> WorldRatingValue {
        Self::Dislike
    }
}
