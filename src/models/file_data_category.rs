use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FileDataCategory {
    #[serde(rename = "multipart")]
    Multipart,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "simple")]
    Simple,
}

impl std::fmt::Display for FileDataCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Multipart => write!(f, "multipart"),
            Self::Queued => write!(f, "queued"),
            Self::Simple => write!(f, "simple"),
        }
    }
}

impl Default for FileDataCategory {
    fn default() -> FileDataCategory {
        Self::Multipart
    }
}
