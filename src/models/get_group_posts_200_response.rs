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
pub struct GetGroupPosts200Response {
    #[serde(rename = "posts", skip_serializing_if = "Option::is_none")]
    pub posts: Option<Vec<models::GroupPost>>,
}

impl GetGroupPosts200Response {
    pub fn new() -> GetGroupPosts200Response {
        GetGroupPosts200Response { posts: None }
    }
}
