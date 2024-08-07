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
pub struct InstanceShortNameResponse {
    #[serde(rename = "secureName")]
    pub secure_name: String,
    #[serde(rename = "shortName", skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
}

impl InstanceShortNameResponse {
    pub fn new(secure_name: String) -> InstanceShortNameResponse {
        InstanceShortNameResponse {
            secure_name,
            short_name: None,
        }
    }
}

