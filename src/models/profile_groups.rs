use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileGroups {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "list", skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<models::ProfileGroup>>,
}

impl ProfileGroups {
    pub fn new() -> ProfileGroups {
        ProfileGroups {
            count: None,
            list: None,
        }
    }
}
