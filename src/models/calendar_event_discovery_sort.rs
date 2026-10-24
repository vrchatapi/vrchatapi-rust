use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CalendarEventDiscoverySort {
    #[serde(rename = "follows_descending")]
    FollowsDescending,
    #[serde(rename = "startTime_ascending")]
    StartTimeAscending,
    #[serde(rename = "startTime_descending")]
    StartTimeDescending,
}

impl std::fmt::Display for CalendarEventDiscoverySort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FollowsDescending => write!(f, "follows_descending"),
            Self::StartTimeAscending => write!(f, "startTime_ascending"),
            Self::StartTimeDescending => write!(f, "startTime_descending"),
        }
    }
}

impl Default for CalendarEventDiscoverySort {
    fn default() -> CalendarEventDiscoverySort {
        Self::FollowsDescending
    }
}
