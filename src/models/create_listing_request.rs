use crate::models;
use serde::{Deserialize, Serialize};

/// CreateListingRequest : Observed create-listing payload fields. Additional fields may exist.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateListingRequest {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "imageId")]
    pub image_id: String,
    #[serde(rename = "listingType")]
    pub listing_type: models::ProductListingType,
    #[serde(rename = "priceTokens")]
    pub price_tokens: i32,
    #[serde(rename = "productIds")]
    pub product_ids: Vec<String>,
    #[serde(rename = "storeIds")]
    pub store_ids: Vec<String>,
}

impl CreateListingRequest {
    /// Observed create-listing payload fields. Additional fields may exist.
    pub fn new(
        description: String,
        display_name: String,
        image_id: String,
        listing_type: models::ProductListingType,
        price_tokens: i32,
        product_ids: Vec<String>,
        store_ids: Vec<String>,
    ) -> CreateListingRequest {
        CreateListingRequest {
            active: None,
            description,
            display_name,
            image_id,
            listing_type,
            price_tokens,
            product_ids,
            store_ids,
        }
    }
}
