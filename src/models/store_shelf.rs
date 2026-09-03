use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoreShelf {
    #[serde(rename = "highlightListing", skip_serializing_if = "Option::is_none")]
    pub highlight_listing: Option<models::ProductListing>,
    #[serde(rename = "highlightListingId", skip_serializing_if = "Option::is_none")]
    pub highlight_listing_id: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "listingIds")]
    pub listing_ids: Vec<String>,
    #[serde(rename = "listings", skip_serializing_if = "Option::is_none")]
    pub listings: Option<Vec<models::ProductListing>>,
    #[serde(
        rename = "shelfBackgroundImageId",
        skip_serializing_if = "Option::is_none"
    )]
    pub shelf_background_image_id: Option<String>,
    #[serde(rename = "shelfDescription")]
    pub shelf_description: String,
    #[serde(rename = "shelfIconImageId", skip_serializing_if = "Option::is_none")]
    pub shelf_icon_image_id: Option<String>,
    #[serde(rename = "shelfLayout")]
    pub shelf_layout: String,
    #[serde(
        rename = "shelfTabBackgroundImageId",
        skip_serializing_if = "Option::is_none"
    )]
    pub shelf_tab_background_image_id: Option<String>,
    #[serde(rename = "shelfTitle")]
    pub shelf_title: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl StoreShelf {
    pub fn new(
        id: String,
        listing_ids: Vec<String>,
        shelf_description: String,
        shelf_layout: String,
        shelf_title: String,
        updated_at: chrono::DateTime<chrono::FixedOffset>,
    ) -> StoreShelf {
        StoreShelf {
            highlight_listing: None,
            highlight_listing_id: None,
            id,
            listing_ids,
            listings: None,
            shelf_background_image_id: None,
            shelf_description,
            shelf_icon_image_id: None,
            shelf_layout,
            shelf_tab_background_image_id: None,
            shelf_title,
            updated_at,
        }
    }
}
