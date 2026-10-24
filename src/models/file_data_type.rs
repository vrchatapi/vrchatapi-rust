use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FileDataType {
    #[serde(rename = "delta")]
    Delta,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "signature")]
    Signature,
}

impl std::fmt::Display for FileDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Delta => write!(f, "delta"),
            Self::File => write!(f, "file"),
            Self::Signature => write!(f, "signature"),
        }
    }
}

impl Default for FileDataType {
    fn default() -> FileDataType {
        Self::Delta
    }
}
