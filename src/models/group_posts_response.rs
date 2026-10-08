use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupPostsResponse {
    #[serde(rename = "posts", skip_serializing_if = "Option::is_none")]
    pub posts: Option<Vec<models::GroupPost>>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

impl GroupPostsResponse {
    pub fn new() -> GroupPostsResponse {
        GroupPostsResponse {
            posts: None,
            total: None,
        }
    }
}
