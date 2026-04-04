use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateProductRequest {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "imageId")]
    pub image_id: String,
    #[serde(rename = "productType")]
    pub product_type: models::ProductType,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "useForSubscriberList")]
    pub use_for_subscriber_list: bool,
}

impl CreateProductRequest {
    pub fn new(
        description: String,
        display_name: String,
        image_id: String,
        product_type: models::ProductType,
        tags: Vec<String>,
        use_for_subscriber_list: bool,
    ) -> CreateProductRequest {
        CreateProductRequest {
            description,
            display_name,
            image_id,
            product_type,
            tags,
            use_for_subscriber_list,
        }
    }
}
