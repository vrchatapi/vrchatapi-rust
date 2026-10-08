use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InventorySpawn {
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "version")]
    pub version: i32,
}

impl InventorySpawn {
    pub fn new(token: String, version: i32) -> InventorySpawn {
        InventorySpawn { token, version }
    }
}
