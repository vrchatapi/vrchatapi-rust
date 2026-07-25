use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateProfile {
    #[serde(rename = "activity")]
    pub activity: models::PrivateProfileActivity,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isFriend")]
    pub is_friend: bool,
    #[serde(rename = "note")]
    pub note: String,
    #[serde(rename = "status")]
    pub status: models::UserStatus,
    #[serde(rename = "statusDescription")]
    pub status_description: String,
}

impl PrivateProfile {
    pub fn new(
        activity: models::PrivateProfileActivity,
        id: String,
        is_friend: bool,
        note: String,
        status: models::UserStatus,
        status_description: String,
    ) -> PrivateProfile {
        PrivateProfile {
            activity,
            id,
            is_friend,
            note,
            status,
            status_description,
        }
    }
}
