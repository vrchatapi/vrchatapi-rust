use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AvatarModerationType {
    #[serde(rename = "block")]
    Block,
}

impl std::fmt::Display for AvatarModerationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Block => write!(f, "block"),
        }
    }
}

impl Default for AvatarModerationType {
    fn default() -> AvatarModerationType {
        Self::Block
    }
}
