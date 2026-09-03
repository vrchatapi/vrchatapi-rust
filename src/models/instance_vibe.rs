use crate::models;
use serde::{Deserialize, Serialize};

/// InstanceVibe : A vibe an instance can be tagged with.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstanceVibe {
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "title")]
    pub title: String,
}

impl InstanceVibe {
    /// A vibe an instance can be tagged with.
    pub fn new(deleted: bool, id: String, title: String) -> InstanceVibe {
        InstanceVibe { deleted, id, title }
    }
}
