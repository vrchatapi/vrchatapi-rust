use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreateWorldRequestReleaseStatus {
    #[serde(rename = "private")]
    Private,
}

impl std::fmt::Display for CreateWorldRequestReleaseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Private => write!(f, "private"),
        }
    }
}

impl Default for CreateWorldRequestReleaseStatus {
    fn default() -> CreateWorldRequestReleaseStatus {
        Self::Private
    }
}
