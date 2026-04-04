use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Store {
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    /// Only for store type world and group
    #[serde(rename = "listingIds", skip_serializing_if = "Option::is_none")]
    pub listing_ids: Option<Vec<String>>,
    /// Only for store type world and group
    #[serde(rename = "listings", skip_serializing_if = "Option::is_none")]
    pub listings: Option<Vec<models::ProductListing>>,
    #[serde(rename = "sellerDisplayName")]
    pub seller_display_name: String,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "sellerId")]
    pub seller_id: String,
    /// Only for store type house
    #[serde(rename = "shelfIds", skip_serializing_if = "Option::is_none")]
    pub shelf_ids: Option<Vec<String>>,
    /// Only for store type house
    #[serde(rename = "shelves", skip_serializing_if = "Option::is_none")]
    pub shelves: Option<Vec<models::StoreShelf>>,
    #[serde(rename = "storeContext", skip_serializing_if = "Option::is_none")]
    pub store_context: Option<models::StoreContext>,
    #[serde(rename = "storeId")]
    pub store_id: String,
    #[serde(rename = "storeStatus", skip_serializing_if = "Option::is_none")]
    pub store_status: Option<String>,
    #[serde(rename = "storeType")]
    pub store_type: models::StoreType,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "worldId", skip_serializing_if = "Option::is_none")]
    pub world_id: Option<String>,
}

impl Store {
    pub fn new(
        description: String,
        display_name: String,
        id: String,
        seller_display_name: String,
        seller_id: String,
        store_id: String,
        store_type: models::StoreType,
        tags: Vec<String>,
    ) -> Store {
        Store {
            created: None,
            description,
            display_name,
            group_id: None,
            id,
            listing_ids: None,
            listings: None,
            seller_display_name,
            seller_id,
            shelf_ids: None,
            shelves: None,
            store_context: None,
            store_id,
            store_status: None,
            store_type,
            tags,
            updated: None,
            world_id: None,
        }
    }
}
