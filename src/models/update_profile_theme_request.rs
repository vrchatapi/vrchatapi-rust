use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateProfileThemeRequest {
    /// Six hexadecimal digits, without a leading `#`.
    #[serde(rename = "buttonColor", skip_serializing_if = "Option::is_none")]
    pub button_color: Option<String>,
    /// Six hexadecimal digits, without a leading `#`.
    #[serde(rename = "iconColor", skip_serializing_if = "Option::is_none")]
    pub icon_color: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Six hexadecimal digits, without a leading `#`.
    #[serde(rename = "subtextColor", skip_serializing_if = "Option::is_none")]
    pub subtext_color: Option<String>,
}

impl UpdateProfileThemeRequest {
    pub fn new() -> UpdateProfileThemeRequest {
        UpdateProfileThemeRequest {
            button_color: None,
            icon_color: None,
            name: None,
            subtext_color: None,
        }
    }
}
