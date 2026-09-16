use crate::models;
use serde::{Deserialize, Serialize};

/// EconomyStatus : Whether the economy is accepting requests.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EconomyStatus {
    #[serde(rename = "economyOnline")]
    pub economy_online: bool,
    #[serde(rename = "economyState")]
    pub economy_state: i32,
}

impl EconomyStatus {
    /// Whether the economy is accepting requests.
    pub fn new(economy_online: bool, economy_state: i32) -> EconomyStatus {
        EconomyStatus {
            economy_online,
            economy_state,
        }
    }
}
