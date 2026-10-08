use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InventoryDefaultAttributesValueValidator {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl InventoryDefaultAttributesValueValidator {
    pub fn new() -> InventoryDefaultAttributesValueValidator {
        InventoryDefaultAttributesValueValidator { r#type: None }
    }
}
