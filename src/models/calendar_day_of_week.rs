use crate::models;
use serde::{Deserialize, Serialize};

/// CalendarDayOfWeek : The day of the week, used for recurring events.
/// The day of the week, used for recurring events.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CalendarDayOfWeek {
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "MO")]
    Mo,
    #[serde(rename = "SA")]
    Sa,
    #[serde(rename = "SU")]
    Su,
    #[serde(rename = "TH")]
    Th,
    #[serde(rename = "TU")]
    Tu,
    #[serde(rename = "WE")]
    We,
}

impl std::fmt::Display for CalendarDayOfWeek {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Fr => write!(f, "FR"),
            Self::Mo => write!(f, "MO"),
            Self::Sa => write!(f, "SA"),
            Self::Su => write!(f, "SU"),
            Self::Th => write!(f, "TH"),
            Self::Tu => write!(f, "TU"),
            Self::We => write!(f, "WE"),
        }
    }
}

impl Default for CalendarDayOfWeek {
    fn default() -> CalendarDayOfWeek {
        Self::Fr
    }
}
