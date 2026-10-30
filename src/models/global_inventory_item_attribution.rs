use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalInventoryItemAttribution {
    #[serde(rename = "creator")]
    pub creator: models::GlobalInventoryItemAttributionCreator,
}

impl GlobalInventoryItemAttribution {
    pub fn new(
        creator: models::GlobalInventoryItemAttributionCreator,
    ) -> GlobalInventoryItemAttribution {
        GlobalInventoryItemAttribution { creator }
    }
}
