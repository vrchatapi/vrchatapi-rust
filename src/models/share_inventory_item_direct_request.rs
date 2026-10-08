use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShareInventoryItemDirectRequest {
    #[serde(rename = "itemId")]
    pub item_id: String,
    #[serde(rename = "users")]
    pub users: Vec<String>,
}

impl ShareInventoryItemDirectRequest {
    pub fn new(item_id: String, users: Vec<String>) -> ShareInventoryItemDirectRequest {
        ShareInventoryItemDirectRequest { item_id, users }
    }
}
