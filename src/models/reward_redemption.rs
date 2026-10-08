use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
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
