use crate::models;
use serde::{Deserialize, Serialize};

/// The categories an instance can be listed under and the vibes it can be tagged with.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstanceOptions {
    #[serde(rename = "categories")]
    pub categories: Vec<models::InstanceCategory>,
    #[serde(rename = "vibes")]
    pub vibes: Vec<models::InstanceVibe>,
}

impl InstanceOptions {
    /// The categories an instance can be listed under and the vibes it can be tagged with.
    pub fn new(
        categories: Vec<models::InstanceCategory>,
        vibes: Vec<models::InstanceVibe>,
    ) -> InstanceOptions {
        InstanceOptions { categories, vibes }
    }
}
