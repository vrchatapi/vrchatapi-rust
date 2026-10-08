use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketFriendLocation {
    #[serde(rename = "canRequestInvite")]
    pub can_request_invite: bool,
    /// Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list.
    #[serde(rename = "location")]
    pub location: String,
    /// Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list.
    #[serde(rename = "travelingToLocation")]
    pub traveling_to_location: String,
    #[serde(rename = "user")]
    pub user: models::User,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "userId")]
    pub user_id: String,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "worldId")]
    pub world_id: String,
}

impl WebsocketFriendLocation {
    pub fn new(
        can_request_invite: bool,
        location: String,
        traveling_to_location: String,
        user: models::User,
        user_id: String,
        world_id: String,
    ) -> WebsocketFriendLocation {
        WebsocketFriendLocation {
            can_request_invite,
            location,
            traveling_to_location,
            user,
            user_id,
            world_id,
        }
    }
}
