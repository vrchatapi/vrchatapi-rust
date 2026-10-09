use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreateAvatarRequestReleaseStatus {
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
}

impl std::fmt::Display for CreateAvatarRequestReleaseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Hidden => write!(f, "hidden"),
            Self::Private => write!(f, "private"),
            Self::Public => write!(f, "public"),
        }
    }
}

impl Default for CreateAvatarRequestReleaseStatus {
    fn default() -> CreateAvatarRequestReleaseStatus {
        Self::Hidden
    }
}
