use crate::models;
use serde::{Deserialize, Serialize};

/// CalendarEventRecurrenceEnd : Details about how a recurring event stops being scheduled
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalendarEventRecurrenceEnd {
    /// Required for \"afterOccurrences\" - The number of times the event will be scheduled before it stops being scheduled
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// Required for \"afterDate\" - The date and time after which the event will stop being scheduled, **without timezone or offset**
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "type")]
    pub r#type: models::CalendarEventRecurrenceEndType,
}

impl CalendarEventRecurrenceEnd {
    /// Details about how a recurring event stops being scheduled
    pub fn new(r#type: models::CalendarEventRecurrenceEndType) -> CalendarEventRecurrenceEnd {
        CalendarEventRecurrenceEnd {
            count: None,
            date: None,
            r#type,
        }
    }
}
