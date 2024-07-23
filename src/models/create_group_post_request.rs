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
pub struct CreateGroupPostRequest {
    /// Post title
    #[serde(rename = "title")]
    pub title: String,
    /// Post text
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "imageId", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// Send notification to group members.
    #[serde(rename = "sendNotification")]
    pub send_notification: bool,
    #[serde(rename = "roleIds", skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<Vec<String>>,
    #[serde(rename = "visibility")]
    pub visibility: models::GroupPostVisibility,
}

impl CreateGroupPostRequest {
    pub fn new(
        title: String,
        text: String,
        send_notification: bool,
        visibility: models::GroupPostVisibility,
    ) -> CreateGroupPostRequest {
        CreateGroupPostRequest {
            title,
            text,
            image_id: None,
            send_notification,
            role_ids: None,
            visibility,
        }
    }
}
