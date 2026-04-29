use crate::models;
use serde::{Deserialize, Serialize};

/// CalendarEventRecurrenceEndType : How a recurring event stops being scheduled
/// How a recurring event stops being scheduled
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CalendarEventRecurrenceEndType {
    #[serde(rename = "afterDate")]
    AfterDate,
    #[serde(rename = "afterOccurrences")]
    AfterOccurrences,
}

impl std::fmt::Display for CalendarEventRecurrenceEndType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AfterDate => write!(f, "afterDate"),
            Self::AfterOccurrences => write!(f, "afterOccurrences"),
        }
    }
}

impl Default for CalendarEventRecurrenceEndType {
    fn default() -> CalendarEventRecurrenceEndType {
        Self::AfterDate
    }
}
