use crate::models;
use serde::{Deserialize, Serialize};

/// ApiConfigConstantsInstance : Instance-related constants
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiConfigConstantsInstance {
    #[serde(
        rename = "POPULATION_BRACKETS",
        skip_serializing_if = "Option::is_none"
    )]
    pub population_brackets: Option<models::ApiConfigConstantsInstancePopulationBrackets>,
}

impl ApiConfigConstantsInstance {
    /// Instance-related constants
    pub fn new() -> ApiConfigConstantsInstance {
        ApiConfigConstantsInstance {
            population_brackets: None,
        }
    }
}
