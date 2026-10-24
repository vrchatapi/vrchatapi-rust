use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataContentSourceInstanceDiscover {
    #[serde(flatten)]
    pub info_push_data_content_source_base: models::InfoPushDataContentSourceBase,
    #[serde(rename = "attributionResponseField")]
    pub attribution_response_field: String,
    #[serde(rename = "endpoint", default)]
    pub endpoint: Endpoint,
    #[serde(rename = "kind", default)]
    pub kind: Kind,
    #[serde(rename = "params")]
    pub params: models::InfoPushDataContentSourceInstanceDiscoverParams,
    #[serde(rename = "responseType", default)]
    pub response_type: ResponseType,
    #[serde(rename = "resultsField", default)]
    pub results_field: ResultsField,
}

impl InfoPushDataContentSourceInstanceDiscover {
    pub fn new(
        info_push_data_content_source_base: models::InfoPushDataContentSourceBase,
        attribution_response_field: String,
        endpoint: Endpoint,
        kind: Kind,
        params: models::InfoPushDataContentSourceInstanceDiscoverParams,
        response_type: ResponseType,
        results_field: ResultsField,
    ) -> InfoPushDataContentSourceInstanceDiscover {
        InfoPushDataContentSourceInstanceDiscover {
            info_push_data_content_source_base,
            attribution_response_field,
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
    #[serde(rename = "/api/1/instances/discover")]
    SlashApiSlash1SlashInstancesSlashDiscover,
}

impl Default for Endpoint {
    fn default() -> Endpoint {
        Self::SlashApiSlash1SlashInstancesSlashDiscover
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "instanceDiscover")]
    InstanceDiscover,
}

impl Default for Kind {
    fn default() -> Kind {
        Self::InstanceDiscover
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ResponseType {
    #[serde(rename = "instance")]
    Instance,
}

impl Default for ResponseType {
    fn default() -> ResponseType {
        Self::Instance
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ResultsField {
    #[serde(rename = "instances")]
    Instances,
}

impl Default for ResultsField {
    fn default() -> ResultsField {
        Self::Instances
    }
}
