use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataContentSourceCalendarDiscoverParams {
    #[serde(rename = "featuredResults")]
    pub featured_results: models::CalendarEventDiscoveryInclusion,
    #[serde(rename = "minimumInterestCount")]
    pub minimum_interest_count: i32,
    #[serde(rename = "requireLinkedInstance")]
    pub require_linked_instance: bool,
    #[serde(rename = "scope")]
    pub scope: models::CalendarEventDiscoveryScope,
    #[serde(rename = "upcomingOffsetMinutes")]
    pub upcoming_offset_minutes: i32,
    #[serde(rename = "upcomingSort")]
    pub upcoming_sort: models::CalendarEventDiscoverySort,
}

impl InfoPushDataContentSourceCalendarDiscoverParams {
    pub fn new(
        featured_results: models::CalendarEventDiscoveryInclusion,
        minimum_interest_count: i32,
        require_linked_instance: bool,
        scope: models::CalendarEventDiscoveryScope,
        upcoming_offset_minutes: i32,
        upcoming_sort: models::CalendarEventDiscoverySort,
    ) -> InfoPushDataContentSourceCalendarDiscoverParams {
        InfoPushDataContentSourceCalendarDiscoverParams {
            featured_results,
            minimum_interest_count,
            require_linked_instance,
            scope,
            upcoming_offset_minutes,
            upcoming_sort,
        }
    }
}
