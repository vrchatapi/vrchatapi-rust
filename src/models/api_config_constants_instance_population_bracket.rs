use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiConfigConstantsInstancePopulationBracket {
    /// Maximum population of an instance in the bracket
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    /// Minimum population of an instance in the bracket
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

impl ApiConfigConstantsInstancePopulationBracket {
    pub fn new() -> ApiConfigConstantsInstancePopulationBracket {
        ApiConfigConstantsInstancePopulationBracket {
            max: None,
            min: None,
        }
    }
}
