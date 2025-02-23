/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiConfigReportOptionsWorld {
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<String>>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<Vec<String>>,
    #[serde(rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
    #[serde(rename = "worldimage", skip_serializing_if = "Option::is_none")]
    pub worldimage: Option<Vec<String>>,
    #[serde(rename = "worldstore", skip_serializing_if = "Option::is_none")]
    pub worldstore: Option<Vec<String>>,
}

impl ApiConfigReportOptionsWorld {
    pub fn new() -> ApiConfigReportOptionsWorld {
        ApiConfigReportOptionsWorld {
            environment: None,
            text: None,
            warnings: None,
            worldimage: None,
            worldstore: None,
        }
    }
}
