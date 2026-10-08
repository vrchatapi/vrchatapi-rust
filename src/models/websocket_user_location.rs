use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketUserLocation {
    /// InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance.
    #[serde(rename = "instance")]
    pub instance: String,
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
}

impl WebsocketUserLocation {
    pub fn new(
        instance: String,
        location: String,
        traveling_to_location: String,
        user: models::User,
        user_id: String,
    ) -> WebsocketUserLocation {
        WebsocketUserLocation {
            instance,
            location,
            traveling_to_location,
            user,
            user_id,
        }
    }
}
