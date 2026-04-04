use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PurchaseContextData {
    #[serde(rename = "locationType")]
    pub location_type: models::ProductPurchaseLocationType,
    #[serde(rename = "storeId", skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "worldId", skip_serializing_if = "Option::is_none")]
    pub world_id: Option<String>,
}

impl PurchaseContextData {
    pub fn new(location_type: models::ProductPurchaseLocationType) -> PurchaseContextData {
        PurchaseContextData {
            location_type,
            store_id: None,
            world_id: None,
        }
    }
}
