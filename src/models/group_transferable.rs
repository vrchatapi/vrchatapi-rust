use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupTransferable {
    #[serde(rename = "requirements")]
    pub requirements: models::GroupTransferableRequirements,
}

impl GroupTransferable {
    pub fn new(requirements: models::GroupTransferableRequirements) -> GroupTransferable {
        GroupTransferable { requirements }
    }
}
