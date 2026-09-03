use crate::models;
use serde::{Deserialize, Serialize};

/// Jam :
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Jam {
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isVisible")]
    pub is_visible: bool,
    #[serde(rename = "moreInfo")]
    pub more_info: String,
    /// One of: - submissions_open - closed
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "stateChangeDates")]
    pub state_change_dates: models::JamStateChangeDates,
    #[serde(
        rename = "submissionContentGateDate",
        deserialize_with = "Option::deserialize"
    )]
    pub submission_content_gate_date: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[serde(rename = "submissionContentGated")]
    pub submission_content_gated: bool,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl Jam {
    pub fn new(
        description: String,
        id: String,
        is_visible: bool,
        more_info: String,
        state: String,
        state_change_dates: models::JamStateChangeDates,
        submission_content_gate_date: Option<chrono::DateTime<chrono::FixedOffset>>,
        submission_content_gated: bool,
        title: String,
        r#type: String,
        updated_at: chrono::DateTime<chrono::FixedOffset>,
    ) -> Jam {
        Jam {
            created_at: None,
            description,
            id,
            is_visible,
            more_info,
            state,
            state_change_dates,
            submission_content_gate_date,
            submission_content_gated,
            title,
            r#type,
            updated_at,
        }
    }
}
