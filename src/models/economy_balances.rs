use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EconomyBalances {
    #[serde(rename = "balance")]
    pub balance: i32,
    #[serde(rename = "earnings", skip_serializing_if = "Option::is_none")]
    pub earnings: Option<i32>,
    #[serde(rename = "standard")]
    pub standard: i32,
}

impl EconomyBalances {
    pub fn new(balance: i32, standard: i32) -> EconomyBalances {
        EconomyBalances {
            balance,
            earnings: None,
            standard,
        }
    }
}
