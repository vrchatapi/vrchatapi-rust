use crate::models;
use serde::{Deserialize, Serialize};

/// Attribution shown alongside a listing.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductListingAttribution {
    #[serde(
        rename = "collaborationId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub collaboration_id: Option<Option<String>>,
    #[serde(rename = "creator", skip_serializing_if = "Option::is_none")]
    pub creator: Option<models::ProductListingAttributionCreator>,
    #[serde(
        rename = "publisher",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher: Option<Option<models::ProductListingAttributionCreator>>,
}

impl ProductListingAttribution {
    /// Attribution shown alongside a listing.
    pub fn new() -> ProductListingAttribution {
        ProductListingAttribution {
            collaboration_id: None,
            creator: None,
            publisher: None,
        }
    }
}
