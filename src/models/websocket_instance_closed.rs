use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketInstanceClosed {
    /// Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list.
    #[serde(rename = "instanceLocation")]
    pub instance_location: String,
}

impl WebsocketInstanceClosed {
    pub fn new(instance_location: String) -> WebsocketInstanceClosed {
        WebsocketInstanceClosed { instance_location }
    }
}
