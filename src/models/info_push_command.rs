use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InfoPushCommand {
    #[serde(rename = "CannedWorldSearch")]
    CannedWorldSearch,
    #[serde(rename = "OpenAccountUpgrade")]
    OpenAccountUpgrade,
    #[serde(rename = "OpenAvatarsMenu")]
    OpenAvatarsMenu,
    #[serde(rename = "OpenHelpArticle")]
    OpenHelpArticle,
    #[serde(rename = "OpenListingDetails")]
    OpenListingDetails,
    #[serde(rename = "OpenSafetyMenu")]
    OpenSafetyMenu,
    #[serde(rename = "OpenURL")]
    OpenUrl,
    #[serde(rename = "OpenVRCPlusMenu")]
    OpenVrcPlusMenu,
    #[serde(rename = "OpenVRChatStore")]
    OpenVrChatStore,
    #[serde(rename = "OpenWorldDetails")]
    OpenWorldDetails,
    #[serde(rename = "OpenWorldsMenu")]
    OpenWorldsMenu,
    #[serde(rename = "scrollToShelf")]
    ScrollToShelf,
}

impl std::fmt::Display for InfoPushCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CannedWorldSearch => write!(f, "CannedWorldSearch"),
            Self::OpenAccountUpgrade => write!(f, "OpenAccountUpgrade"),
            Self::OpenAvatarsMenu => write!(f, "OpenAvatarsMenu"),
            Self::OpenHelpArticle => write!(f, "OpenHelpArticle"),
            Self::OpenListingDetails => write!(f, "OpenListingDetails"),
            Self::OpenSafetyMenu => write!(f, "OpenSafetyMenu"),
            Self::OpenUrl => write!(f, "OpenURL"),
            Self::OpenVrcPlusMenu => write!(f, "OpenVRCPlusMenu"),
            Self::OpenVrChatStore => write!(f, "OpenVRChatStore"),
            Self::OpenWorldDetails => write!(f, "OpenWorldDetails"),
            Self::OpenWorldsMenu => write!(f, "OpenWorldsMenu"),
            Self::ScrollToShelf => write!(f, "scrollToShelf"),
        }
    }
}

impl Default for InfoPushCommand {
    fn default() -> InfoPushCommand {
        Self::CannedWorldSearch
    }
}
