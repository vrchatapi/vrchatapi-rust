use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateProfileActivity {
    /// InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance.
    #[serde(rename = "instanceId", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// Either a date-time or an empty string.
    #[serde(rename = "last_activity", skip_serializing_if = "Option::is_none")]
    pub last_activity: Option<String>,
    /// Either a date-time or an empty string.
    #[serde(rename = "last_login", skip_serializing_if = "Option::is_none")]
    pub last_login: Option<String>,
    /// Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// This is normally `android`, `ios`, `standalonewindows`, `web`, or the empty value ``, but also supposedly can be any random Unity version such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`.
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<models::UserState>,
    #[serde(
        rename = "travelingToInstance",
        skip_serializing_if = "Option::is_none"
    )]
    pub traveling_to_instance: Option<String>,
    #[serde(
        rename = "travelingToLocation",
        skip_serializing_if = "Option::is_none"
    )]
    pub traveling_to_location: Option<String>,
    #[serde(rename = "travelingToWorld", skip_serializing_if = "Option::is_none")]
    pub traveling_to_world: Option<String>,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "worldId", skip_serializing_if = "Option::is_none")]
    pub world_id: Option<String>,
}

impl PrivateProfileActivity {
    pub fn new() -> PrivateProfileActivity {
        PrivateProfileActivity {
            instance_id: None,
            last_activity: None,
            last_login: None,
            location: None,
            platform: None,
            state: None,
            traveling_to_instance: None,
            traveling_to_location: None,
            traveling_to_world: None,
            world_id: None,
        }
    }
}
