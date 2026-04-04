use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductPurchaseHistory {
    #[serde(rename = "endDate")]
    pub end_date: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "transactions")]
    pub transactions: Vec<models::ProductPurchaseRecord>,
}

impl ProductPurchaseHistory {
    pub fn new(
        end_date: String,
        start_date: String,
        transactions: Vec<models::ProductPurchaseRecord>,
    ) -> ProductPurchaseHistory {
        ProductPurchaseHistory {
            end_date,
            start_date,
            transactions,
        }
    }
}
