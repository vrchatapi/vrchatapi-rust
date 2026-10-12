use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiConfigConstantsInstance {
    /// Population brackets based on instance population
    #[serde(
        rename = "POPULATION_BRACKETS",
        skip_serializing_if = "Option::is_none"
    )]
    pub population_brackets: Option<models::ApiConfigConstantsInstancePopulationBrackets>,
}

impl ApiConfigConstantsInstance {
    pub fn new() -> ApiConfigConstantsInstance {
        ApiConfigConstantsInstance {
            population_brackets: None,
        }
    }
}
