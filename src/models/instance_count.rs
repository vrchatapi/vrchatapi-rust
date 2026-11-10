use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstanceCount {
    #[serde(rename = "asOf")]
    pub as_of: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "events")]
    pub events: i32,
    #[serde(rename = "friendIcons")]
    pub friend_icons: Vec<serde_json::Value>,
    #[serde(rename = "friends")]
    pub friends: i32,
    /// The ID of a `ContentShelf` entry, whose results `getInfoPushContent` returns.
    #[serde(rename = "friendsEntryId")]
    pub friends_entry_id: String,
    #[serde(rename = "instances")]
    pub instances: i32,
}

impl InstanceCount {
    pub fn new(
        as_of: chrono::DateTime<chrono::FixedOffset>,
        events: i32,
        friend_icons: Vec<serde_json::Value>,
        friends: i32,
        friends_entry_id: String,
        instances: i32,
    ) -> InstanceCount {
        InstanceCount {
            as_of,
            events,
            friend_icons,
            friends,
            friends_entry_id,
            instances,
        }
    }
}
