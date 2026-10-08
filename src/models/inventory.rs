use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
    #[serde(rename = "data")]
    pub data: Vec<models::InventoryItem>,
    #[serde(rename = "totalCount")]
    pub total_count: i32,
}

impl Inventory {
    pub fn new(data: Vec<models::InventoryItem>, total_count: i32) -> Inventory {
        Inventory { data, total_count }
    }
}
