use crate::models;
use serde::{Deserialize, Serialize};

/// ServiceStatus : Status information for a service request
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceStatus {
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// The id of this service, NOT the id of the thing this service was requested for.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "progress")]
    pub progress: Vec<serde_json::Value>,
    /// The id of the user who requested this service.
    #[serde(rename = "requesterUserId")]
    pub requester_user_id: String,
    #[serde(rename = "state")]
    pub state: String,
    /// The id of the thing this service was requested for.
    #[serde(rename = "subjectId")]
    pub subject_id: String,
    /// The kind of the thing this service was requested for.
    #[serde(rename = "subjectType")]
    pub subject_type: String,
    /// The kind of service that was requested.
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ServiceStatus {
    /// Status information for a service request
    pub fn new(
        created_at: chrono::DateTime<chrono::FixedOffset>,
        id: String,
        progress: Vec<serde_json::Value>,
        requester_user_id: String,
        state: String,
        subject_id: String,
        subject_type: String,
        r#type: String,
        updated_at: chrono::DateTime<chrono::FixedOffset>,
    ) -> ServiceStatus {
        ServiceStatus {
            created_at,
            id,
            progress,
            requester_user_id,
            state,
            subject_id,
            subject_type,
            r#type,
            updated_at,
        }
    }
}
