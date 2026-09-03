use crate::models;
use serde::{Deserialize, Serialize};

/// InstanceCategory : A category an instance can be listed under.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstanceCategory {
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "order")]
    pub order: i32,
}

impl InstanceCategory {
    /// A category an instance can be listed under.
    pub fn new(
        deleted: bool,
        icon_url: String,
        id: String,
        name: String,
        order: i32,
    ) -> InstanceCategory {
        InstanceCategory {
            deleted,
            icon_url,
            id,
            name,
            order,
        }
    }
}
