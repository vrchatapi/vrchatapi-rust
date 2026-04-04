use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RewardRedemption {
    #[serde(rename = "data")]
    pub data: models::RewardRedemptionData,
    /// One of `badge`, `item`, ...
    #[serde(rename = "type")]
    pub r#type: String,
}

impl RewardRedemption {
    pub fn new(data: models::RewardRedemptionData, r#type: String) -> RewardRedemption {
        RewardRedemption { data, r#type }
    }
}
