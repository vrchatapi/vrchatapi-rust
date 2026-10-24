use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataContentSourceCalendarDiscover {
    #[serde(flatten)]
    pub info_push_data_content_source_base: models::InfoPushDataContentSourceBase,
    #[serde(rename = "endpoint", default)]
    pub endpoint: Endpoint,
    #[serde(rename = "kind", default)]
    pub kind: Kind,
    #[serde(rename = "params")]
    pub params: models::InfoPushDataContentSourceCalendarDiscoverParams,
    #[serde(rename = "responseType", default)]
    pub response_type: ResponseType,
    #[serde(rename = "resultsField", default)]
    pub results_field: ResultsField,
}

impl InfoPushDataContentSourceCalendarDiscover {
    pub fn new(
        info_push_data_content_source_base: models::InfoPushDataContentSourceBase,
        endpoint: Endpoint,
        kind: Kind,
        params: models::InfoPushDataContentSourceCalendarDiscoverParams,
        response_type: ResponseType,
        results_field: ResultsField,
    ) -> InfoPushDataContentSourceCalendarDiscover {
        InfoPushDataContentSourceCalendarDiscover {
            info_push_data_content_source_base,
            endpoint,
            kind,
            params,
            response_type,
            results_field,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Endpoint {
    #[serde(rename = "/api/1/calendar/discover")]
    SlashApiSlash1SlashCalendarSlashDiscover,
}

impl Default for Endpoint {
    fn default() -> Endpoint {
        Self::SlashApiSlash1SlashCalendarSlashDiscover
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "calendarDiscover")]
    CalendarDiscover,
}

impl Default for Kind {
    fn default() -> Kind {
        Self::CalendarDiscover
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ResponseType {
    #[serde(rename = "calendarEntry")]
    CalendarEntry,
}

impl Default for ResponseType {
    fn default() -> ResponseType {
        Self::CalendarEntry
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ResultsField {
    #[serde(rename = "results")]
    Results,
}

impl Default for ResultsField {
    fn default() -> ResultsField {
        Self::Results
    }
}
