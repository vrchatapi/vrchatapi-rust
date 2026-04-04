use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductPurchaseProduct {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "imageId", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(
        rename = "licenseId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub license_id: Option<Option<String>>,
    #[serde(rename = "productType")]
    pub product_type: models::ProductType,
}

impl ProductPurchaseProduct {
    pub fn new(
        display_name: String,
        id: String,
        product_type: models::ProductType,
    ) -> ProductPurchaseProduct {
        ProductPurchaseProduct {
            display_name,
            id,
            image_id: None,
            license_id: None,
            product_type,
        }
    }
}
