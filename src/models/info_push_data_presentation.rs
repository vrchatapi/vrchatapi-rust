use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataPresentation {
    #[serde(rename = "layout")]
    pub layout: String,
    #[serde(rename = "pinnedDescription", skip_serializing_if = "Option::is_none")]
    pub pinned_description: Option<String>,
    #[serde(rename = "socialDisplay", skip_serializing_if = "Option::is_none")]
    pub social_display: Option<String>,
}

impl InfoPushDataPresentation {
    pub fn new(layout: String) -> InfoPushDataPresentation {
        InfoPushDataPresentation {
            layout,
            pinned_description: None,
            social_display: None,
        }
    }
}
