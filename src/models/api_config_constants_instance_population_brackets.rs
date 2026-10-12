use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiConfigConstantsInstancePopulationBrackets {
    /// Crowded population range
    #[serde(rename = "CROWDED", skip_serializing_if = "Option::is_none")]
    pub crowded: Option<models::ApiConfigConstantsInstancePopulationBracket>,
    /// Few population range
    #[serde(rename = "FEW", skip_serializing_if = "Option::is_none")]
    pub few: Option<models::ApiConfigConstantsInstancePopulationBracket>,
    /// Many population range
    #[serde(rename = "MANY", skip_serializing_if = "Option::is_none")]
    pub many: Option<models::ApiConfigConstantsInstancePopulationBracket>,
}

impl ApiConfigConstantsInstancePopulationBrackets {
    pub fn new() -> ApiConfigConstantsInstancePopulationBrackets {
        ApiConfigConstantsInstancePopulationBrackets {
            crowded: None,
            few: None,
            many: None,
        }
    }
}
