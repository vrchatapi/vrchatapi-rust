use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PurchaseProductListingRequest {
    #[serde(rename = "contextData", skip_serializing_if = "Option::is_none")]
    pub context_data: Option<models::PurchaseContextData>,
    #[serde(rename = "listingId")]
    pub listing_id: String,
    #[serde(rename = "listingVariantId", skip_serializing_if = "Option::is_none")]
    pub listing_variant_id: Option<String>,
    #[serde(rename = "quantity")]
    pub quantity: i32,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "receiverId", skip_serializing_if = "Option::is_none")]
    pub receiver_id: Option<String>,
    #[serde(rename = "stackable", skip_serializing_if = "Option::is_none")]
    pub stackable: Option<bool>,
    #[serde(rename = "totalPrice")]
    pub total_price: i32,
}

impl PurchaseProductListingRequest {
    pub fn new(
        listing_id: String,
        quantity: i32,
        total_price: i32,
    ) -> PurchaseProductListingRequest {
        PurchaseProductListingRequest {
            context_data: None,
            listing_id,
            listing_variant_id: None,
            quantity,
            receiver_id: None,
            stackable: None,
            total_price,
        }
    }
}
