use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalendarEventDiscovery {
    /// Base64-encoded JSON:   type: object   properties:     dataSource:       type: string       enum:         - featured         - personalized     dataIndex:       type: integer       format: int32     phase:       type: string       enum:         - all         - live         - upcoming       description: see CalendarEventDiscoveryScope     asOf:       type: integer       format: int64       description: milliseconds since Unix epoch     paramHash:       type: string       format: string       description: Base64-encoded 256-bit hash of the original query parameters
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
