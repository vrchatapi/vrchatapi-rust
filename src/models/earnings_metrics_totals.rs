use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EarningsMetricsTotals {
    #[serde(rename = "otpEarnings")]
    pub otp_earnings: i32,
    #[serde(rename = "otpPurchaseCount")]
    pub otp_purchase_count: i32,
    #[serde(rename = "subscriberEarnings")]
    pub subscriber_earnings: i32,
    #[serde(rename = "subscriberMonths")]
    pub subscriber_months: i32,
    #[serde(rename = "totalEarnings")]
    pub total_earnings: i32,
}

impl EarningsMetricsTotals {
    pub fn new(
        otp_earnings: i32,
        otp_purchase_count: i32,
        subscriber_earnings: i32,
        subscriber_months: i32,
        total_earnings: i32,
    ) -> EarningsMetricsTotals {
        EarningsMetricsTotals {
            otp_earnings,
            otp_purchase_count,
            subscriber_earnings,
            subscriber_months,
            total_earnings,
        }
    }
}
