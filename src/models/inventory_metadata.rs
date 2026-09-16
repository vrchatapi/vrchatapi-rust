use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InventoryMetadata {
    #[serde(rename = "animated", skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
    #[serde(rename = "animationStyle", skip_serializing_if = "Option::is_none")]
    pub animation_style: Option<String>,
    #[serde(rename = "assetBundleId", skip_serializing_if = "Option::is_none")]
    pub asset_bundle_id: Option<String>,
    #[serde(rename = "assets", skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<models::InventoryAsset>>,
    #[serde(rename = "fileId", skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// Six hexadecimal digits, without a leading `#`. May be empty.
    #[serde(rename = "gradientEnd", skip_serializing_if = "Option::is_none")]
    pub gradient_end: Option<String>,
    /// Six hexadecimal digits, without a leading `#`. May be empty.
    #[serde(rename = "gradientStart", skip_serializing_if = "Option::is_none")]
    pub gradient_start: Option<String>,
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// Only in bundles
    #[serde(
        rename = "inventoryItemsToInstantiate",
        skip_serializing_if = "Option::is_none"
    )]
    pub inventory_items_to_instantiate: Option<Vec<String>>,
    #[serde(rename = "maskTag", skip_serializing_if = "Option::is_none")]
    pub mask_tag: Option<String>,
    #[serde(rename = "propId", skip_serializing_if = "Option::is_none")]
    pub prop_id: Option<String>,
    #[serde(rename = "propKind", skip_serializing_if = "Option::is_none")]
    pub prop_kind: Option<i32>,
    #[serde(rename = "viewfinderBundleId", skip_serializing_if = "Option::is_none")]
    pub viewfinder_bundle_id: Option<String>,
}

impl InventoryMetadata {
    pub fn new() -> InventoryMetadata {
        InventoryMetadata {
            animated: None,
            animation_style: None,
            asset_bundle_id: None,
            assets: None,
            file_id: None,
            gradient_end: None,
            gradient_start: None,
            image_url: None,
            inventory_items_to_instantiate: None,
            mask_tag: None,
            prop_id: None,
            prop_kind: None,
            viewfinder_bundle_id: None,
        }
    }
}
