use crate::models;
use serde::{Deserialize, Serialize};

/// GroupMemberLimitedUser : Only visible via the /groups/:groupId/members endpoint, **not** when fetching a specific user.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupMemberLimitedUser {
    #[serde(rename = "bannerColor", skip_serializing_if = "Option::is_none")]
    pub banner_color: Option<String>,
    #[serde(rename = "bannerType", skip_serializing_if = "Option::is_none")]
    pub banner_type: Option<String>,
    #[serde(rename = "bannerUrl", skip_serializing_if = "Option::is_none")]
    pub banner_url: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "iconFrame", skip_serializing_if = "Option::is_none")]
    pub icon_frame: Option<String>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "nameplateEffect", skip_serializing_if = "Option::is_none")]
    pub nameplate_effect: Option<String>,
    #[serde(rename = "profileEffect", skip_serializing_if = "Option::is_none")]
    pub profile_effect: Option<String>,
}

impl GroupMemberLimitedUser {
    /// Only visible via the /groups/:groupId/members endpoint, **not** when fetching a specific user.
    pub fn new() -> GroupMemberLimitedUser {
        GroupMemberLimitedUser {
            banner_color: None,
            banner_type: None,
            banner_url: None,
            display_name: None,
            icon_frame: None,
            icon_url: None,
            id: None,
            nameplate_effect: None,
            profile_effect: None,
        }
    }
}
