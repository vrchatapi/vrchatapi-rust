use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateProfile {
    #[serde(rename = "activity", skip_serializing_if = "Option::is_none")]
    pub activity: Option<models::PrivateProfileActivity>,
    /// State of a friend request between the caller and this user. VRChat sends the string `\"null\"`, not JSON `null`.
    #[serde(
        rename = "friendRequestStatus",
        skip_serializing_if = "Option::is_none"
    )]
    pub friend_request_status: Option<String>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isFriend", skip_serializing_if = "Option::is_none")]
    pub is_friend: Option<bool>,
    #[serde(
        rename = "note",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub note: Option<Option<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::UserStatus>,
    #[serde(rename = "statusDescription", skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
}

impl PrivateProfile {
    pub fn new() -> PrivateProfile {
        PrivateProfile {
            activity: None,
            friend_request_status: None,
            id: None,
            is_friend: None,
            note: None,
            status: None,
            status_description: None,
        }
    }
}
