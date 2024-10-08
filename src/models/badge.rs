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
pub struct Badge {
    /// only present in CurrentUser badges
    #[serde(
        rename = "assignedAt",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_at: Option<Option<String>>,
    #[serde(rename = "badgeDescription")]
    pub badge_description: String,
    #[serde(rename = "badgeId")]
    pub badge_id: String,
    /// direct url to image
    #[serde(rename = "badgeImageUrl")]
    pub badge_image_url: String,
    #[serde(rename = "badgeName")]
    pub badge_name: String,
    /// only present in CurrentUser badges
    #[serde(
        rename = "hidden",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub hidden: Option<Option<bool>>,
    #[serde(rename = "showcased")]
    pub showcased: bool,
    /// only present in CurrentUser badges
    #[serde(
        rename = "updatedAt",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<Option<String>>,
}

impl Badge {
    pub fn new(
        badge_description: String,
        badge_id: String,
        badge_image_url: String,
        badge_name: String,
        showcased: bool,
    ) -> Badge {
        Badge {
            assigned_at: None,
            badge_description,
            badge_id,
            badge_image_url,
            badge_name,
            hidden: None,
            showcased,
            updated_at: None,
        }
    }
}
