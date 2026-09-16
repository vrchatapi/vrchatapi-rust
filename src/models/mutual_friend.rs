use crate::models;
use serde::{Deserialize, Serialize};

/// MutualFriend : User object received when querying mutual friends
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MutualFriend {
    /// Hex colour without a leading `#`.
    #[serde(rename = "bannerColor", skip_serializing_if = "Option::is_none")]
    pub banner_color: Option<String>,
    #[serde(rename = "bannerType", skip_serializing_if = "Option::is_none")]
    pub banner_type: Option<String>,
    #[serde(rename = "bannerUrl", skip_serializing_if = "Option::is_none")]
    pub banner_url: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "iconFrame", skip_serializing_if = "Option::is_none")]
    pub icon_frame: Option<String>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "nameplateEffect", skip_serializing_if = "Option::is_none")]
    pub nameplate_effect: Option<String>,
    #[serde(rename = "profileEffect", skip_serializing_if = "Option::is_none")]
    pub profile_effect: Option<String>,
    #[serde(rename = "status")]
    pub status: models::UserStatus,
    #[serde(rename = "statusDescription")]
    pub status_description: String,
}

impl MutualFriend {
    /// User object received when querying mutual friends
    pub fn new(
        display_name: String,
        id: String,
        status: models::UserStatus,
        status_description: String,
    ) -> MutualFriend {
        MutualFriend {
            banner_color: None,
            banner_type: None,
            banner_url: None,
            display_name,
            icon_frame: None,
            icon_url: None,
            id,
            nameplate_effect: None,
            profile_effect: None,
            status,
            status_description,
        }
    }
}
