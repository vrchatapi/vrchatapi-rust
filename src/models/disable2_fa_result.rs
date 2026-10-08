use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Disable2FaResult {
    #[serde(rename = "removed")]
    pub removed: bool,
}

impl Disable2FaResult {
    pub fn new(removed: bool) -> Disable2FaResult {
        Disable2FaResult { removed }
    }
}
