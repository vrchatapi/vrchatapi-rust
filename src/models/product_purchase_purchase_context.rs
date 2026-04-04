use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductPurchasePurchaseContext {
    #[serde(rename = "locationType", skip_serializing_if = "Option::is_none")]
    pub location_type: Option<String>,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "worldId", skip_serializing_if = "Option::is_none")]
    pub world_id: Option<String>,
    #[serde(rename = "worldName", skip_serializing_if = "Option::is_none")]
    pub world_name: Option<String>,
}

impl ProductPurchasePurchaseContext {
    pub fn new() -> ProductPurchasePurchaseContext {
        ProductPurchasePurchaseContext {
            location_type: None,
            world_id: None,
            world_name: None,
        }
    }
}
