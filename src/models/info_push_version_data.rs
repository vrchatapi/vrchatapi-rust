use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushVersionData {
    #[serde(rename = "version")]
    pub version: String,
}

impl InfoPushVersionData {
    pub fn new(version: String) -> InfoPushVersionData {
        InfoPushVersionData { version }
    }
}
