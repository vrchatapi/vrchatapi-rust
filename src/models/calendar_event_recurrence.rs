use crate::models;
use serde::{Deserialize, Serialize};

/// CalendarEventRecurrence : Details about how a recurring event will be scheduled. If the event is to be scheduled indefinitely, this will lack an \"end\" property.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalendarEventRecurrence {
    /// Which days of the week the event will be scheduled, only valid/present for \"weekly\" recurring events
    #[serde(rename = "daysOfWeek", skip_serializing_if = "Option::is_none")]
    pub days_of_week: Option<Vec<models::CalendarDayOfWeek>>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<models::CalendarEventRecurrenceEnd>,
    #[serde(rename = "frequency")]
    pub frequency: models::CalendarEventFrequency,
    /// How often the event will be scheduled, in units of \"frequency\"
    #[serde(rename = "interval")]
    pub interval: i32,
    /// The timezone the event will be scheduled in, in Area/Location format
    #[serde(rename = "timezone")]
    pub timezone: String,
}

impl CalendarEventRecurrence {
    /// Details about how a recurring event will be scheduled. If the event is to be scheduled indefinitely, this will lack an \"end\" property.
    pub fn new(
        frequency: models::CalendarEventFrequency,
        interval: i32,
        timezone: String,
    ) -> CalendarEventRecurrence {
        CalendarEventRecurrence {
            days_of_week: None,
            end: None,
            frequency,
            interval,
            timezone,
        }
    }
}
