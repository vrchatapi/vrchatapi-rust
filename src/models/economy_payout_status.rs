use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EconomyPayoutStatus {
    #[serde(rename = "accountId")]
    pub account_id: i32,
    #[serde(rename = "activePayout", skip_serializing_if = "Option::is_none")]
    pub active_payout: Option<models::EconomyPayout>,
    #[serde(rename = "activePayoutCancellable")]
    pub active_payout_cancellable: bool,
    #[serde(rename = "activePayoutTiliaAmount")]
    pub active_payout_tilia_amount: i32,
    #[serde(rename = "earningsBalance")]
    pub earnings_balance: i32,
    #[serde(rename = "eligibility")]
    pub eligibility: models::EconomyPayoutEligibility,
    #[serde(rename = "payoutEligible")]
    pub payout_eligible: bool,
    #[serde(rename = "tiliaId")]
    pub tilia_id: String,
}

impl EconomyPayoutStatus {
    pub fn new(
        account_id: i32,
        active_payout_cancellable: bool,
        active_payout_tilia_amount: i32,
        earnings_balance: i32,
        eligibility: models::EconomyPayoutEligibility,
        payout_eligible: bool,
        tilia_id: String,
    ) -> EconomyPayoutStatus {
        EconomyPayoutStatus {
            account_id,
            active_payout: None,
            active_payout_cancellable,
            active_payout_tilia_amount,
            earnings_balance,
            eligibility,
            payout_eligible,
            tilia_id,
        }
    }
}
