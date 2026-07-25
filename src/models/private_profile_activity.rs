use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateProfileActivity {
    /// InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance.
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    /// Either a date-time or an empty string.
    #[serde(rename = "last_activity")]
    pub last_activity: String,
    /// Either a date-time or an empty string.
    #[serde(rename = "last_login")]
    pub last_login: String,
    /// Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list.
    #[serde(rename = "location")]
    pub location: String,
    /// This is normally `android`, `ios`, `standalonewindows`, `web`, or the empty value ``, but also supposedly can be any random Unity version such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`.
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "state")]
    pub state: models::UserState,
    #[serde(rename = "travelingToInstance")]
    pub traveling_to_instance: String,
    #[serde(rename = "travelingToLocation")]
    pub traveling_to_location: String,
    #[serde(rename = "travelingToWorld")]
    pub traveling_to_world: String,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "worldId")]
    pub world_id: String,
}

impl PrivateProfileActivity {
    pub fn new(
        instance_id: String,
        last_activity: String,
        last_login: String,
        location: String,
        platform: String,
        state: models::UserState,
        traveling_to_instance: String,
        traveling_to_location: String,
        traveling_to_world: String,
        world_id: String,
    ) -> PrivateProfileActivity {
        PrivateProfileActivity {
            instance_id,
            last_activity,
            last_login,
            location,
            platform,
            state,
            traveling_to_instance,
            traveling_to_location,
            traveling_to_world,
            world_id,
        }
    }
}
