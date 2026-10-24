use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstanceCount {
    #[serde(rename = "asOf")]
    pub as_of: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "events")]
    pub events: i32,
    #[serde(rename = "friends")]
    pub friends: i32,
    #[serde(rename = "instances")]
    pub instances: i32,
}

impl InstanceCount {
    pub fn new(
        as_of: chrono::DateTime<chrono::FixedOffset>,
        events: i32,
        friends: i32,
        instances: i32,
    ) -> InstanceCount {
        InstanceCount {
            as_of,
            events,
            friends,
            instances,
        }
    }
}
