use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BetaUserField {
    #[serde(rename = "allowedValues", skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
    #[serde(
        rename = "excludeFromAnalytics",
        skip_serializing_if = "Option::is_none"
    )]
    pub exclude_from_analytics: Option<bool>,
    #[serde(rename = "required")]
    pub required: bool,
}

impl BetaUserField {
    pub fn new(required: bool) -> BetaUserField {
        BetaUserField {
            allowed_values: None,
            exclude_from_analytics: None,
            required,
        }
    }
}
