use crate::models;
use serde::{Deserialize, Serialize};

/// InfoPush :
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPush {
    #[serde(
        rename = "clientMinVersion",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_min_version: Option<Option<serde_json::Value>>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "data")]
    pub data: models::InfoPushData,
    #[serde(
        rename = "endDate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub end_date: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    #[serde(rename = "experiment", skip_serializing_if = "Option::is_none")]
    pub experiment: Option<models::InfoPushExperiment>,
    /// Unknown usage, MD5
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "priority")]
    pub priority: i32,
    #[serde(rename = "regions", skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
    #[serde(rename = "releaseStatus")]
    pub release_status: models::ReleaseStatus,
    #[serde(rename = "requireClientTags", skip_serializing_if = "Option::is_none")]
    pub require_client_tags: Option<Vec<String>>,
    #[serde(
        rename = "startDate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub start_date: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl InfoPush {
    pub fn new(
        created_at: chrono::DateTime<chrono::FixedOffset>,
        data: models::InfoPushData,
        hash: String,
        id: String,
        is_enabled: bool,
        priority: i32,
        release_status: models::ReleaseStatus,
        tags: Vec<String>,
        updated_at: chrono::DateTime<chrono::FixedOffset>,
    ) -> InfoPush {
        InfoPush {
            client_min_version: None,
            created_at,
            data,
            end_date: None,
            experiment: None,
            hash,
            id,
            is_enabled,
            priority,
            regions: None,
            release_status,
            require_client_tags: None,
            start_date: None,
            tags,
            updated_at,
        }
    }
}
