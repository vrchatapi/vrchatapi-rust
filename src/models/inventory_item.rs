use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InventoryItem {
    #[serde(rename = "acquisition", skip_serializing_if = "Option::is_none")]
    pub acquisition: Option<String>,
    #[serde(
        rename = "attribution",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub attribution: Option<Option<serde_json::Value>>,
    #[serde(rename = "collections")]
    pub collections: Vec<String>,
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "defaultAttributes")]
    pub default_attributes:
        std::collections::HashMap<String, models::InventoryDefaultAttributesValue>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "equipSlot", skip_serializing_if = "Option::is_none")]
    pub equip_slot: Option<models::InventoryEquipSlot>,
    #[serde(rename = "equipSlots", skip_serializing_if = "Option::is_none")]
    pub equip_slots: Option<Vec<models::InventoryEquipSlot>>,
    #[serde(
        rename = "expiryDate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_date: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    #[serde(rename = "flags")]
    pub flags: Vec<String>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "holderId")]
    pub holder_id: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
    #[serde(rename = "isSeen")]
    pub is_seen: bool,
    #[serde(rename = "itemType")]
    pub item_type: models::InventoryItemType,
    #[serde(rename = "itemTypeLabel")]
    pub item_type_label: String,
    #[serde(rename = "last_equipped", skip_serializing_if = "Option::is_none")]
    pub last_equipped: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "metadata")]
    pub metadata: models::InventoryMetadata,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "quantifiable")]
    pub quantifiable: bool,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "templateId")]
    pub template_id: String,
    #[serde(rename = "template_created_at")]
    pub template_created_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "template_updated_at")]
    pub template_updated_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "userAttributes")]
    pub user_attributes: models::InventoryUserAttributes,
    #[serde(rename = "validateUserAttributes")]
    pub validate_user_attributes: bool,
}

impl InventoryItem {
    pub fn new(
        collections: Vec<String>,
        created_at: chrono::DateTime<chrono::FixedOffset>,
        default_attributes: std::collections::HashMap<
            String,
            models::InventoryDefaultAttributesValue,
        >,
        description: String,
        flags: Vec<String>,
        holder_id: String,
        id: String,
        image_url: String,
        is_archived: bool,
        is_seen: bool,
        item_type: models::InventoryItemType,
        item_type_label: String,
        metadata: models::InventoryMetadata,
        name: String,
        quantifiable: bool,
        tags: Vec<String>,
        template_id: String,
        template_created_at: chrono::DateTime<chrono::FixedOffset>,
        template_updated_at: chrono::DateTime<chrono::FixedOffset>,
        updated_at: chrono::DateTime<chrono::FixedOffset>,
        user_attributes: models::InventoryUserAttributes,
        validate_user_attributes: bool,
    ) -> InventoryItem {
        InventoryItem {
            acquisition: None,
            attribution: None,
            collections,
            created_at,
            default_attributes,
            description,
            equip_slot: None,
            equip_slots: None,
            expiry_date: None,
            flags,
            holder_id,
            id,
            image_url,
            is_archived,
            is_seen,
            item_type,
            item_type_label,
            last_equipped: None,
            metadata,
            name,
            quantifiable,
            tags,
            template_id,
            template_created_at,
            template_updated_at,
            updated_at,
            user_attributes,
            validate_user_attributes,
        }
    }
}
