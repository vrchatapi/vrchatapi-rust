use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mutuals {
    #[serde(rename = "friends")]
    pub friends: i32,
    #[serde(rename = "groups")]
    pub groups: i32,
}

impl Mutuals {
    pub fn new(friends: i32, groups: i32) -> Mutuals {
        Mutuals { friends, groups }
    }
}
