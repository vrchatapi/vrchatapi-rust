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
pub struct AvatarStyles {
    #[serde(
        rename = "primary",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary: Option<Option<String>>,
    #[serde(
        rename = "secondary",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub secondary: Option<Option<String>>,
    #[serde(rename = "supplementary", skip_serializing_if = "Option::is_none")]
    pub supplementary: Option<Vec<String>>,
}

impl AvatarStyles {
    pub fn new() -> AvatarStyles {
        AvatarStyles {
            primary: None,
            secondary: None,
            supplementary: None,
        }
    }
}
