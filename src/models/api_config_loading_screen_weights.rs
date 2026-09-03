use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiConfigLoadingScreenWeights {
    #[serde(rename = "announcement", skip_serializing_if = "Option::is_none")]
    pub announcement: Option<i32>,
    #[serde(rename = "informational", skip_serializing_if = "Option::is_none")]
    pub informational: Option<i32>,
    #[serde(rename = "promotional", skip_serializing_if = "Option::is_none")]
    pub promotional: Option<i32>,
}

impl ApiConfigLoadingScreenWeights {
    pub fn new() -> ApiConfigLoadingScreenWeights {
        ApiConfigLoadingScreenWeights {
            announcement: None,
            informational: None,
            promotional: None,
        }
    }
}
