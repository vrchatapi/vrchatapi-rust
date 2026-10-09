use crate::models;
use serde::{Deserialize, Serialize};

/// Few population range
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiConfigConstantsInstancePopulationBracketsFew {
    /// Maximum population for a few instance
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    /// Minimum population for a few instance
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

impl ApiConfigConstantsInstancePopulationBracketsFew {
    /// Few population range
    pub fn new() -> ApiConfigConstantsInstancePopulationBracketsFew {
        ApiConfigConstantsInstancePopulationBracketsFew {
            max: None,
            min: None,
        }
    }
}
