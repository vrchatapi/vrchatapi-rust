use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalendarEventDiscovery {
    /// Pass back as the `nextCursor` query parameter to read the page after this one.
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "results")]
    pub results: Vec<models::CalendarEvent>,
}

impl CalendarEventDiscovery {
    pub fn new(results: Vec<models::CalendarEvent>) -> CalendarEventDiscovery {
        CalendarEventDiscovery {
            next_cursor: None,
            results,
        }
    }
}
