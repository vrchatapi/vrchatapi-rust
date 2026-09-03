use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductListing {
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "attribution", skip_serializing_if = "Option::is_none")]
    pub attribution: Option<models::ProductListingAttribution>,
    #[serde(rename = "buyerRefundable")]
    pub buyer_refundable: bool,
    #[serde(
        rename = "collabUserDisplayName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub collab_user_display_name: Option<Option<String>>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "collabUserId", skip_serializing_if = "Option::is_none")]
    pub collab_user_id: Option<String>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(
        rename = "duration",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub duration: Option<Option<i32>>,
    #[serde(
        rename = "durationType",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub duration_type: Option<Option<String>>,
    #[serde(rename = "groupIcon", skip_serializing_if = "Option::is_none")]
    pub group_icon: Option<String>,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(
        rename = "groupName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub group_name: Option<Option<String>>,
    #[serde(rename = "hasAvatar")]
    pub has_avatar: bool,
    #[serde(rename = "hasCompanion", skip_serializing_if = "Option::is_none")]
    pub has_companion: Option<bool>,
    #[serde(rename = "hasInventory", skip_serializing_if = "Option::is_none")]
    pub has_inventory: Option<bool>,
    #[serde(rename = "hasUdon")]
    pub has_udon: bool,
    #[serde(rename = "hydratedProducts", skip_serializing_if = "Option::is_none")]
    pub hydrated_products: Option<Vec<models::Product>>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "imageId", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(
        rename = "imageUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_url: Option<Option<String>>,
    #[serde(rename = "listingType")]
    pub listing_type: models::ProductListingType,
    #[serde(rename = "listingVariants", skip_serializing_if = "Option::is_none")]
    pub listing_variants: Option<Vec<models::ProductListingVariant>>,
    #[serde(rename = "permanent", skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
    #[serde(rename = "priceTokens")]
    pub price_tokens: i32,
    #[serde(rename = "productIds")]
    pub product_ids: Vec<String>,
    #[serde(rename = "productType")]
    pub product_type: models::ProductType,
    #[serde(rename = "productTypes", skip_serializing_if = "Option::is_none")]
    pub product_types: Option<Vec<String>>,
    /// Product ids. The products themselves arrive in `hydratedProducts`.
    #[serde(rename = "products")]
    pub products: Vec<String>,
    #[serde(rename = "purchaseCount", skip_serializing_if = "Option::is_none")]
    pub purchase_count: Option<i32>,
    #[serde(
        rename = "purchaseCountQuantity",
        skip_serializing_if = "Option::is_none"
    )]
    pub purchase_count_quantity: Option<i32>,
    #[serde(rename = "quantifiable", skip_serializing_if = "Option::is_none")]
    pub quantifiable: Option<bool>,
    #[serde(rename = "recurrable")]
    pub recurrable: bool,
    #[serde(rename = "refundable")]
    pub refundable: bool,
    #[serde(rename = "sellerDisplayName")]
    pub seller_display_name: String,
    #[serde(rename = "sellerId")]
    pub seller_id: String,
    #[serde(rename = "soldByVrc", skip_serializing_if = "Option::is_none")]
    pub sold_by_vrc: Option<bool>,
    #[serde(rename = "stackable")]
    pub stackable: bool,
    #[serde(rename = "storeIds")]
    pub store_ids: Vec<String>,
    #[serde(rename = "subtitle", skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[serde(
        rename = "vrcPlusDiscountPrice",
        skip_serializing_if = "Option::is_none"
    )]
    pub vrc_plus_discount_price: Option<i32>,
    #[serde(
        rename = "whenToExpire",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub when_to_expire: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
}

impl ProductListing {
    pub fn new(
        active: bool,
        buyer_refundable: bool,
        description: String,
        display_name: String,
        has_avatar: bool,
        has_udon: bool,
        id: String,
        listing_type: models::ProductListingType,
        price_tokens: i32,
        product_ids: Vec<String>,
        product_type: models::ProductType,
        products: Vec<String>,
        recurrable: bool,
        refundable: bool,
        seller_display_name: String,
        seller_id: String,
        stackable: bool,
        store_ids: Vec<String>,
    ) -> ProductListing {
        ProductListing {
            active,
            archived: None,
            attribution: None,
            buyer_refundable,
            collab_user_display_name: None,
            collab_user_id: None,
            created: None,
            description,
            display_name,
            duration: None,
            duration_type: None,
            group_icon: None,
            group_id: None,
            group_name: None,
            has_avatar,
            has_companion: None,
            has_inventory: None,
            has_udon,
            hydrated_products: None,
            id,
            image_id: None,
            image_url: None,
            listing_type,
            listing_variants: None,
            permanent: None,
            price_tokens,
            product_ids,
            product_type,
            product_types: None,
            products,
            purchase_count: None,
            purchase_count_quantity: None,
            quantifiable: None,
            recurrable,
            refundable,
            seller_display_name,
            seller_id,
            sold_by_vrc: None,
            stackable,
            store_ids,
            subtitle: None,
            tags: None,
            updated: None,
            vrc_plus_discount_price: None,
            when_to_expire: None,
        }
    }
}
