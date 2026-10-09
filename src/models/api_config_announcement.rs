use crate::models;
use serde::{Deserialize, Serialize};

/// Public Announcement
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiConfigAnnouncement {
    /// Announcement name
    #[serde(rename = "name")]
    pub name: String,
    /// Announcement text
    #[serde(rename = "text")]
    pub text: String,
}

impl ApiConfigAnnouncement {
    /// Public Announcement
    pub fn new(name: String, text: String) -> ApiConfigAnnouncement {
        ApiConfigAnnouncement { name, text }
    }
}
