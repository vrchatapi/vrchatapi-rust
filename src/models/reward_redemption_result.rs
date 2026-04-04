use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RewardRedemptionResult {
    #[serde(rename = "redeemedRewards")]
    pub redeemed_rewards: Vec<models::RewardRedemption>,
    #[serde(rename = "redemptionCode")]
    pub redemption_code: String,
}

impl RewardRedemptionResult {
    pub fn new(
        redeemed_rewards: Vec<models::RewardRedemption>,
        redemption_code: String,
    ) -> RewardRedemptionResult {
        RewardRedemptionResult {
            redeemed_rewards,
            redemption_code,
        }
    }
}
