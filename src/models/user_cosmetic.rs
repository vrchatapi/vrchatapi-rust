use crate::models;
use serde::{Deserialize, Serialize};

/// UserCosmetic : A cosmetic a user holds, without the template's presentation fields.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCosmetic {
    #[serde(rename = "acquiredOn")]
    pub acquired_on: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "acquisition")]
    pub acquisition: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "itemType")]
    pub item_type: models::InventoryItemType,
    #[serde(rename = "templateId")]
    pub template_id: String,
    #[serde(rename = "userAttributes")]
    pub user_attributes: models::InventoryUserAttributes,
}

impl UserCosmetic {
    /// A cosmetic a user holds, without the template's presentation fields.
    pub fn new(
        acquired_on: chrono::DateTime<chrono::FixedOffset>,
        acquisition: String,
        id: String,
        item_type: models::InventoryItemType,
        template_id: String,
        user_attributes: models::InventoryUserAttributes,
    ) -> UserCosmetic {
        UserCosmetic {
            acquired_on,
            acquisition,
            id,
            item_type,
            template_id,
            user_attributes,
        }
    }
}
