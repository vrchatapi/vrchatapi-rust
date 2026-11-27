use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HypeTrainLevel {
    #[serde(rename = "giftsNeededToUnlock")]
    pub gifts_needed_to_unlock: i32,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "reached")]
    pub reached: bool,
}

impl HypeTrainLevel {
    pub fn new(gifts_needed_to_unlock: i32, level: i32, reached: bool) -> HypeTrainLevel {
        HypeTrainLevel {
            gifts_needed_to_unlock,
            level,
            reached,
        }
    }
}
