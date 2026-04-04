use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RewardRedemptionRequest {
    #[serde(rename = "code")]
    pub code: String,
}

impl RewardRedemptionRequest {
    pub fn new(code: String) -> RewardRedemptionRequest {
        RewardRedemptionRequest { code }
    }
}
