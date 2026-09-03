use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JamStateChangeDates {
    #[serde(
        rename = "closed",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub closed: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    #[serde(
        rename = "submissionsClosed",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub submissions_closed: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    #[serde(
        rename = "submissionsOpened",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub submissions_opened: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    #[serde(
        rename = "winnersSelected",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub winners_selected: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
}

impl JamStateChangeDates {
    pub fn new() -> JamStateChangeDates {
        JamStateChangeDates {
            closed: None,
            submissions_closed: None,
            submissions_opened: None,
            winners_selected: None,
        }
    }
}
