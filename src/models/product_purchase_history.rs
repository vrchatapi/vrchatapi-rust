use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductPurchaseHistory {
    #[serde(rename = "endDate")]
    pub end_date: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "startDate")]
    pub start_date: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "transactions")]
    pub transactions: Vec<models::ProductPurchaseRecord>,
}

impl ProductPurchaseHistory {
    pub fn new(
        end_date: chrono::DateTime<chrono::FixedOffset>,
        start_date: chrono::DateTime<chrono::FixedOffset>,
        transactions: Vec<models::ProductPurchaseRecord>,
    ) -> ProductPurchaseHistory {
        ProductPurchaseHistory {
            end_date,
            start_date,
            transactions,
        }
    }
}
