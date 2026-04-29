use crate::models;
use serde::{Deserialize, Serialize};

/// CalendarEventOccurrenceKind : How an event occurs or recurs. \"single\" is a standalone event, \"series\" is a parent specification of a recurring series, and \"occurrence\" is an individual event in a series.
/// How an event occurs or recurs. \"single\" is a standalone event, \"series\" is a parent specification of a recurring series, and \"occurrence\" is an individual event in a series.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CalendarEventOccurrenceKind {
    #[serde(rename = "occurrence")]
    Occurrence,
    #[serde(rename = "series")]
    Series,
    #[serde(rename = "single")]
    Single,
}

impl std::fmt::Display for CalendarEventOccurrenceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Occurrence => write!(f, "occurrence"),
            Self::Series => write!(f, "series"),
            Self::Single => write!(f, "single"),
        }
    }
}

impl Default for CalendarEventOccurrenceKind {
    fn default() -> CalendarEventOccurrenceKind {
        Self::Occurrence
    }
}
