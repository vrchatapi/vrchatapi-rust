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
pub struct Verify2FaResult {
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl Verify2FaResult {
    pub fn new(verified: bool) -> Verify2FaResult {
        Verify2FaResult {
            verified,
            enabled: None,
        }
    }
}
