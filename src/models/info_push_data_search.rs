use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataSearch {
    #[serde(rename = "searchContent", skip_serializing_if = "Option::is_none")]
    pub search_content: Option<String>,
    #[serde(rename = "searchInFields", skip_serializing_if = "Option::is_none")]
    pub search_in_fields: Option<String>,
    #[serde(rename = "searchQuery", skip_serializing_if = "Option::is_none")]
    pub search_query: Option<String>,
    #[serde(rename = "searchTags", skip_serializing_if = "Option::is_none")]
    pub search_tags: Option<String>,
    #[serde(rename = "sortBy", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl InfoPushDataSearch {
    pub fn new() -> InfoPushDataSearch {
        InfoPushDataSearch {
            search_content: None,
            search_in_fields: None,
            search_query: None,
            search_tags: None,
            sort_by: None,
        }
    }
}
