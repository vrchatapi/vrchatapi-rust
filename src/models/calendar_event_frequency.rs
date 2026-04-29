use crate::models;
use serde::{Deserialize, Serialize};

/// CalendarEventFrequency : The time unit used to specify how often a recurring event occurs.
/// The time unit used to specify how often a recurring event occurs.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CalendarEventFrequency {
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "yearly")]
    Yearly,
}

impl std::fmt::Display for CalendarEventFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Daily => write!(f, "daily"),
            Self::Monthly => write!(f, "monthly"),
            Self::Weekly => write!(f, "weekly"),
            Self::Yearly => write!(f, "yearly"),
        }
    }
}

impl Default for CalendarEventFrequency {
    fn default() -> CalendarEventFrequency {
        Self::Daily
    }
}
