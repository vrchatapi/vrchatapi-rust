use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiConfigIosVersion {
    #[serde(rename = "major")]
    pub major: i32,
    #[serde(rename = "minor")]
    pub minor: i32,
}

impl ApiConfigIosVersion {
    pub fn new(major: i32, minor: i32) -> ApiConfigIosVersion {
        ApiConfigIosVersion { major, minor }
    }
}
