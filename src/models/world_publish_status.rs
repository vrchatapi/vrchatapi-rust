use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorldPublishStatus {
    #[serde(rename = "canPublish")]
    pub can_publish: bool,
}

impl WorldPublishStatus {
    pub fn new(can_publish: bool) -> WorldPublishStatus {
        WorldPublishStatus { can_publish }
    }
}
