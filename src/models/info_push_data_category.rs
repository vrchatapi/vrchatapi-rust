use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataCategory {
    #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(rename = "ipsQuery", skip_serializing_if = "Option::is_none")]
    pub ips_query: Option<models::InfoPushIpsQuery>,
    #[serde(rename = "maxCells", skip_serializing_if = "Option::is_none")]
    pub max_cells: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<models::InfoPushDataCategoryName>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl InfoPushDataCategory {
    pub fn new() -> InfoPushDataCategory {
        InfoPushDataCategory {
            ids: None,
            ips_query: None,
            max_cells: None,
            name: None,
            r#type: None,
        }
    }
}
