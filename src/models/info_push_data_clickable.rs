use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataClickable {
    /// In case of OpenURL, this would contain the link.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
    #[serde(rename = "command")]
    pub command: Command,
}

impl InfoPushDataClickable {
    pub fn new(command: Command) -> InfoPushDataClickable {
        InfoPushDataClickable {
            parameters: None,
            command,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Command {
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
}

impl Default for Command {
    fn default() -> Command {
        Self::CannedWorldSearch
    }
}
