use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductPurchaseRecord {
    #[serde(rename = "amount")]
    pub amount: i32,
    #[serde(rename = "balance")]
    pub balance: i32,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "fromUserDisplayName")]
    pub from_user_display_name: String,
    #[serde(rename = "listingDisplayName")]
    pub listing_display_name: String,
    #[serde(rename = "listingType")]
    pub listing_type: models::ProductListingType,
    /// Where (first- or third-party) the purchase was made
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "purchaseId")]
    pub purchase_id: String,
    #[serde(rename = "reason")]
    pub reason: i32,
    #[serde(rename = "reasonLabel")]
    pub reason_label: String,
    #[serde(rename = "transactionId")]
    pub transaction_id: i32,
    #[serde(rename = "transactionLineId")]
    pub transaction_line_id: i32,
}

impl ProductPurchaseRecord {
    pub fn new(
        amount: i32,
        balance: i32,
        date: String,
        from_user_display_name: String,
        listing_display_name: String,
        listing_type: models::ProductListingType,
        platform: String,
        purchase_id: String,
        reason: i32,
        reason_label: String,
        transaction_id: i32,
        transaction_line_id: i32,
    ) -> ProductPurchaseRecord {
        ProductPurchaseRecord {
            amount,
            balance,
            date,
            from_user_display_name,
            listing_display_name,
            listing_type,
            platform,
            purchase_id,
            reason,
            reason_label,
            transaction_id,
            transaction_line_id,
        }
    }
}
