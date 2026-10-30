use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalInventoryItemAttributionCreator {
    #[serde(rename = "customName")]
    pub custom_name: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl GlobalInventoryItemAttributionCreator {
    pub fn new(custom_name: String, user_id: String) -> GlobalInventoryItemAttributionCreator {
        GlobalInventoryItemAttributionCreator {
            custom_name,
            user_id,
        }
    }
}
