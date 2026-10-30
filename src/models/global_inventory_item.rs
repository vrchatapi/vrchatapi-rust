use crate::models;
use serde::{Deserialize, Serialize};

/// An inventory item flagged `global`. It belongs to no account, so it carries no holder, quantity or timestamps.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalInventoryItem {
    #[serde(rename = "acquisition")]
    pub acquisition: String,
    #[serde(rename = "attribution")]
    pub attribution: models::GlobalInventoryItemAttribution,
    #[serde(rename = "collections")]
    pub collections: Vec<String>,
    #[serde(rename = "defaultAttributes")]
    pub default_attributes:
        std::collections::HashMap<String, models::InventoryDefaultAttributesValue>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "equipSlot")]
    pub equip_slot: String,
    #[serde(rename = "equipSlots")]
    pub equip_slots: Vec<String>,
    #[serde(rename = "flags")]
    pub flags: Vec<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
    #[serde(rename = "isSeen")]
    pub is_seen: bool,
    #[serde(rename = "itemType")]
    pub item_type: String,
    #[serde(rename = "itemTypeLabel")]
    pub item_type_label: String,
    #[serde(rename = "metadata")]
    pub metadata: models::InventoryMetadata,
    #[serde(rename = "name")]
    pub name: String,
    /// An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance.
    #[serde(rename = "productId", deserialize_with = "Option::deserialize")]
    pub product_id: Option<serde_json::Value>,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "templateId")]
    pub template_id: String,
    #[serde(rename = "userAttributes")]
    pub user_attributes: std::collections::HashMap<String, models::InventoryDefaultAttributesValue>,
    #[serde(rename = "validateUserAttributes")]
    pub validate_user_attributes: bool,
}

impl GlobalInventoryItem {
    /// An inventory item flagged `global`. It belongs to no account, so it carries no holder, quantity or timestamps.
    pub fn new(
        acquisition: String,
        attribution: models::GlobalInventoryItemAttribution,
        collections: Vec<String>,
        default_attributes: std::collections::HashMap<
            String,
            models::InventoryDefaultAttributesValue,
        >,
        description: String,
        equip_slot: String,
        equip_slots: Vec<String>,
        flags: Vec<String>,
        id: String,
        image_url: String,
        is_archived: bool,
        is_seen: bool,
        item_type: String,
        item_type_label: String,
        metadata: models::InventoryMetadata,
        name: String,
        product_id: Option<serde_json::Value>,
        tags: Vec<String>,
        template_id: String,
        user_attributes: std::collections::HashMap<String, models::InventoryDefaultAttributesValue>,
        validate_user_attributes: bool,
    ) -> GlobalInventoryItem {
        GlobalInventoryItem {
            acquisition,
            attribution,
            collections,
            default_attributes,
            description,
            equip_slot,
            equip_slots,
            flags,
            id,
            image_url,
            is_archived,
            is_seen,
            item_type,
            item_type_label,
            metadata,
            name,
            product_id,
            tags,
            template_id,
            user_attributes,
            validate_user_attributes,
        }
    }
}
