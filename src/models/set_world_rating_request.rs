use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetWorldRatingRequest {
    #[serde(rename = "value")]
    pub value: models::WorldRatingValue,
}

impl SetWorldRatingRequest {
    pub fn new(value: models::WorldRatingValue) -> SetWorldRatingRequest {
        SetWorldRatingRequest { value }
    }
}
