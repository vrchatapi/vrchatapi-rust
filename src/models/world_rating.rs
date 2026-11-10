use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorldRating {
    #[serde(rename = "value", deserialize_with = "Option::deserialize")]
    pub value: Option<models::WorldRatingValue>,
}

impl WorldRating {
    pub fn new(value: Option<models::WorldRatingValue>) -> WorldRating {
        WorldRating { value }
    }
}
