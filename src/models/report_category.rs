use crate::models;
use serde::{Deserialize, Serialize};

/// ReportCategory : A category used for reporting content
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportCategory {
    /// The description of the report category
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ipsArticle", skip_serializing_if = "Option::is_none")]
    pub ips_article: Option<String>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// The label of the report category
    #[serde(rename = "text")]
    pub text: String,
    /// The title of the report category
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The tooltip that describes the category
    #[serde(rename = "tooltip")]
    pub tooltip: String,
}

impl ReportCategory {
    /// A category used for reporting content
    pub fn new(text: String, tooltip: String) -> ReportCategory {
        ReportCategory {
            description: None,
            ips_article: None,
            order: None,
            text,
            title: None,
            tooltip,
        }
    }
}
