use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Beta {
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "betaAppId", skip_serializing_if = "Option::is_none")]
    pub beta_app_id: Option<String>,
    #[serde(rename = "betaGroupId", skip_serializing_if = "Option::is_none")]
    pub beta_group_id: Option<String>,
    #[serde(rename = "betaName")]
    pub beta_name: String,
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "lastSynchronizedAt", skip_serializing_if = "Option::is_none")]
    pub last_synchronized_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    /// The fields a registration must supply, keyed by field name.
    #[serde(rename = "userFields")]
    pub user_fields: std::collections::HashMap<String, models::BetaUserField>,
}

impl Beta {
    pub fn new(
        active: bool,
        beta_name: String,
        created_at: chrono::DateTime<chrono::FixedOffset>,
        id: String,
        r#type: String,
        updated_at: chrono::DateTime<chrono::FixedOffset>,
        user_fields: std::collections::HashMap<String, models::BetaUserField>,
    ) -> Beta {
        Beta {
            active,
            beta_app_id: None,
            beta_group_id: None,
            beta_name,
            created_at,
            id,
            last_synchronized_at: None,
            r#type,
            updated_at,
            user_fields,
        }
    }
}
