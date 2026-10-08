use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropPublishStatus {
    #[serde(rename = "canPublish", skip_serializing_if = "Option::is_none")]
    pub can_publish: Option<bool>,
}

impl PropPublishStatus {
    pub fn new() -> PropPublishStatus {
        PropPublishStatus { can_publish: None }
    }
}
