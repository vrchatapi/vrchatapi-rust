use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushContentCalendarEntries {
    /// Pass back as the `nextCursor` query parameter to read the page after this one.
    #[serde(rename = "nextCursor", deserialize_with = "Option::deserialize")]
    pub next_cursor: Option<String>,
    #[serde(rename = "responseType", default)]
    pub response_type: ResponseType,
    #[serde(rename = "results")]
    pub results: Vec<models::CalendarEvent>,
}

impl InfoPushContentCalendarEntries {
    pub fn new(
        next_cursor: Option<String>,
        response_type: ResponseType,
        results: Vec<models::CalendarEvent>,
    ) -> InfoPushContentCalendarEntries {
        InfoPushContentCalendarEntries {
            next_cursor,
            response_type,
            results,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ResponseType {
    #[serde(rename = "calendarEntry")]
    CalendarEntry,
}

impl Default for ResponseType {
    fn default() -> ResponseType {
        Self::CalendarEntry
    }
}
