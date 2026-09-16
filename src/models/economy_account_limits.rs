use crate::models;
use serde::{Deserialize, Serialize};

/// EconomyAccountLimits : Returned only when `getLimits` is set.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EconomyAccountLimits {
    #[serde(rename = "buyingTokenMaxPerDay")]
    pub buying_token_max_per_day: i32,
    #[serde(rename = "buyingTokenRemainingAllowed")]
    pub buying_token_remaining_allowed: i32,
}

impl EconomyAccountLimits {
    /// Returned only when `getLimits` is set.
    pub fn new(
        buying_token_max_per_day: i32,
        buying_token_remaining_allowed: i32,
    ) -> EconomyAccountLimits {
        EconomyAccountLimits {
            buying_token_max_per_day,
            buying_token_remaining_allowed,
        }
    }
}
