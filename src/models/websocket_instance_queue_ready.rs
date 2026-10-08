use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketInstanceQueueReady {
    #[serde(rename = "expiry")]
    pub expiry: chrono::DateTime<chrono::FixedOffset>,
    /// Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list.
    #[serde(rename = "location")]
    pub location: String,
}

impl WebsocketInstanceQueueReady {
    pub fn new(
        expiry: chrono::DateTime<chrono::FixedOffset>,
        location: String,
    ) -> WebsocketInstanceQueueReady {
        WebsocketInstanceQueueReady { expiry, location }
    }
}
