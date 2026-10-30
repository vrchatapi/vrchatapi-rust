use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EquipInventoryItemRequest {
    #[serde(rename = "equipSlot")]
    pub equip_slot: String,
}

impl EquipInventoryItemRequest {
    pub fn new(equip_slot: String) -> EquipInventoryItemRequest {
        EquipInventoryItemRequest { equip_slot }
    }
}
