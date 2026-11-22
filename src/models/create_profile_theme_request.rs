use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateProfileThemeRequest {
    /// Six hexadecimal digits, without a leading `#`.
    #[serde(rename = "buttonColor")]
    pub button_color: String,
    /// Six hexadecimal digits, without a leading `#`.
    #[serde(rename = "iconColor")]
    pub icon_color: String,
    #[serde(rename = "name")]
    pub name: String,
    /// Six hexadecimal digits, without a leading `#`.
    #[serde(rename = "subtextColor")]
    pub subtext_color: String,
}

impl CreateProfileThemeRequest {
    pub fn new(
        button_color: String,
        icon_color: String,
        name: String,
        subtext_color: String,
    ) -> CreateProfileThemeRequest {
        CreateProfileThemeRequest {
            button_color,
            icon_color,
            name,
            subtext_color,
        }
    }
}
