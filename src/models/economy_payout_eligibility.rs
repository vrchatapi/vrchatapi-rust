use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EconomyPayoutEligibility {
    #[serde(rename = "issue")]
    pub issue: String,
    #[serde(rename = "okBalance")]
    pub ok_balance: bool,
    #[serde(rename = "okFrequency")]
    pub ok_frequency: bool,
    #[serde(rename = "okNotOngoing")]
    pub ok_not_ongoing: bool,
    #[serde(rename = "okStanding")]
    pub ok_standing: bool,
}

impl EconomyPayoutEligibility {
    pub fn new(
        issue: String,
        ok_balance: bool,
        ok_frequency: bool,
        ok_not_ongoing: bool,
        ok_standing: bool,
    ) -> EconomyPayoutEligibility {
        EconomyPayoutEligibility {
            issue,
            ok_balance,
            ok_frequency,
            ok_not_ongoing,
            ok_standing,
        }
    }
}
