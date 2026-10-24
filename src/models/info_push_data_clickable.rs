use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataClickable {
    /// In case of OpenURL, this would contain the link.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
    #[serde(rename = "command")]
    pub command: models::InfoPushCommand,
}

impl InfoPushDataClickable {
    pub fn new(command: models::InfoPushCommand) -> InfoPushDataClickable {
        InfoPushDataClickable {
            parameters: None,
            command,
        }
    }
}
