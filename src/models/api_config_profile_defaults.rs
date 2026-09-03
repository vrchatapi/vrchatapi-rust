use crate::models;
use serde::{Deserialize, Serialize};

/// ApiConfigProfileDefaults : Default profile theme colours, each a hex RGB triplet without a leading `#`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiConfigProfileDefaults {
    #[serde(
        rename = "backgroundGradientBottom",
        skip_serializing_if = "Option::is_none"
    )]
    pub background_gradient_bottom: Option<String>,
    #[serde(
        rename = "backgroundGradientTop",
        skip_serializing_if = "Option::is_none"
    )]
    pub background_gradient_top: Option<String>,
    #[serde(rename = "themeButtonColor", skip_serializing_if = "Option::is_none")]
    pub theme_button_color: Option<String>,
    #[serde(rename = "themeIconColor", skip_serializing_if = "Option::is_none")]
    pub theme_icon_color: Option<String>,
    #[serde(rename = "themeSubtextColor", skip_serializing_if = "Option::is_none")]
    pub theme_subtext_color: Option<String>,
}

impl ApiConfigProfileDefaults {
    /// Default profile theme colours, each a hex RGB triplet without a leading `#`.
    pub fn new() -> ApiConfigProfileDefaults {
        ApiConfigProfileDefaults {
            background_gradient_bottom: None,
            background_gradient_top: None,
            theme_button_color: None,
            theme_icon_color: None,
            theme_subtext_color: None,
        }
    }
}
