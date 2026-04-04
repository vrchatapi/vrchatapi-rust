use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EconomyBalances {
    #[serde(rename = "balance")]
    pub balance: i32,
    #[serde(rename = "earnings")]
    pub earnings: i32,
    #[serde(rename = "standard")]
    pub standard: i32,
}

impl EconomyBalances {
    pub fn new(balance: i32, earnings: i32, standard: i32) -> EconomyBalances {
        EconomyBalances {
            balance,
            earnings,
            standard,
        }
    }
}
