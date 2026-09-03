use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InventoryTemplate {
    #[serde(
        rename = "attribution",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub attribution: Option<Option<serde_json::Value>>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "collections")]
    pub collections: Vec<String>,
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "defaultAttributes")]
    pub default_attributes: serde_json::Value,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "dropStatus", skip_serializing_if = "Option::is_none")]
    pub drop_status: Option<String>,
    #[serde(rename = "equipSlots")]
    pub equip_slots: Vec<String>,
    #[serde(rename = "flags")]
    pub flags: Vec<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    #[serde(rename = "itemType")]
    pub item_type: models::InventoryItemType,
    #[serde(rename = "itemTypeLabel")]
    pub item_type_label: String,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<models::InventoryMetadata>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(
        rename = "notificationDetails",
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_details: Option<models::InventoryNotificationDetails>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "validateUserAttributes")]
    pub validate_user_attributes: bool,
}

impl InventoryTemplate {
    pub fn new(
        author_id: String,
        collections: Vec<String>,
        created_at: chrono::DateTime<chrono::FixedOffset>,
        default_attributes: serde_json::Value,
        description: String,
        equip_slots: Vec<String>,
        flags: Vec<String>,
        id: String,
        image_url: String,
        item_type: models::InventoryItemType,
        item_type_label: String,
        name: String,
        tags: Vec<String>,
        updated_at: chrono::DateTime<chrono::FixedOffset>,
        validate_user_attributes: bool,
    ) -> InventoryTemplate {
        InventoryTemplate {
            attribution: None,
            author_id,
            collections,
            created_at,
            default_attributes,
            description,
            drop_status: None,
            equip_slots,
            flags,
            id,
            image_url,
            item_type,
            item_type_label,
            metadata: None,
            name,
            notification_details: None,
            status: None,
            tags,
            updated_at,
            validate_user_attributes,
        }
    }
}
