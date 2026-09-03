use crate::models;
use serde::{Deserialize, Serialize};

/// ProductListingAttribution : Attribution shown alongside a listing.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductListingAttribution {
    #[serde(rename = "creator", skip_serializing_if = "Option::is_none")]
    pub creator: Option<models::ProductListingAttributionCreator>,
    #[serde(rename = "publisher", skip_serializing_if = "Option::is_none")]
    pub publisher: Option<models::ProductListingAttributionCreator>,
}

impl ProductListingAttribution {
    /// Attribution shown alongside a listing.
    pub fn new() -> ProductListingAttribution {
        ProductListingAttribution {
            creator: None,
            publisher: None,
        }
    }
}
