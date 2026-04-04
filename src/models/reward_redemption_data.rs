use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RewardRedemptionData {
    #[serde(rename = "badge", skip_serializing_if = "Option::is_none")]
    pub badge: Option<models::RewardBadge>,
    #[serde(rename = "item", skip_serializing_if = "Option::is_none")]
    pub item: Option<models::InventoryTemplate>,
}

impl RewardRedemptionData {
    pub fn new() -> RewardRedemptionData {
        RewardRedemptionData {
            badge: None,
            item: None,
        }
    }
}
