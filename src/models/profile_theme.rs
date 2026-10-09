use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileTheme {
    /// Six hexadecimal digits, without a leading `#`. May be empty.
    #[serde(rename = "buttonColor")]
    pub button_color: String,
    /// Six hexadecimal digits, without a leading `#`. May be empty.
    #[serde(rename = "iconColor")]
    pub icon_color: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    /// Six hexadecimal digits, without a leading `#`. May be empty.
    #[serde(rename = "subtextColor")]
    pub subtext_color: String,
}

impl ProfileTheme {
    pub fn new(
        button_color: String,
        icon_color: String,
        id: String,
        name: String,
        subtext_color: String,
    ) -> ProfileTheme {
        ProfileTheme {
            button_color,
            icon_color,
            id,
            name,
            subtext_color,
        }
    }
}
