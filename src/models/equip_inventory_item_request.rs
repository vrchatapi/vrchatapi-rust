use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EquipInventoryItemRequest {
    #[serde(rename = "equipSlot")]
    pub equip_slot: models::InventoryEquipSlot,
}

impl EquipInventoryItemRequest {
    pub fn new(equip_slot: models::InventoryEquipSlot) -> EquipInventoryItemRequest {
        EquipInventoryItemRequest { equip_slot }
    }
}
