use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EarningsMetrics {
    #[serde(rename = "breakdown")]
    pub breakdown: Vec<serde_json::Value>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "sellerId")]
    pub seller_id: String,
    #[serde(rename = "totals")]
    pub totals: models::EarningsMetricsTotals,
}

impl EarningsMetrics {
    pub fn new(
        breakdown: Vec<serde_json::Value>,
        seller_id: String,
        totals: models::EarningsMetricsTotals,
    ) -> EarningsMetrics {
        EarningsMetrics {
            breakdown,
            seller_id,
            totals,
        }
    }
}
