use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EconomyPayoutList {
    #[serde(rename = "payouts")]
    pub payouts: Vec<models::EconomyPayout>,
}

impl EconomyPayoutList {
    pub fn new(payouts: Vec<models::EconomyPayout>) -> EconomyPayoutList {
        EconomyPayoutList { payouts }
    }
}
