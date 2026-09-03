use crate::models;
use serde::{Deserialize, Serialize};

/// SsoProvider : A third-party service VRChat mints an SSO token for. Anything else is refused with \"That is not a supported SSO provider.\"
/// A third-party service VRChat mints an SSO token for. Anything else is refused with \"That is not a supported SSO provider.\"
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SsoProvider {
    #[serde(rename = "canny")]
    Canny,
    #[serde(rename = "furality")]
    Furality,
}

impl std::fmt::Display for SsoProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Canny => write!(f, "canny"),
            Self::Furality => write!(f, "furality"),
        }
    }
}

impl Default for SsoProvider {
    fn default() -> SsoProvider {
        Self::Canny
    }
}
