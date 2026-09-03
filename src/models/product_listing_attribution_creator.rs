use crate::models;
use serde::{Deserialize, Serialize};

/// ProductListingAttributionCreator : The creator credited on a listing.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductListingAttributionCreator {
    #[serde(rename = "customName", skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl ProductListingAttributionCreator {
    /// The creator credited on a listing.
    pub fn new() -> ProductListingAttributionCreator {
        ProductListingAttributionCreator {
            custom_name: None,
            user_id: None,
        }
    }
}
