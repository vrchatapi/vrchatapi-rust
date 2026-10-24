use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataContentSourceUnknown {
    #[serde(flatten)]
    pub info_push_data_content_source_base: models::InfoPushDataContentSourceBase,
    #[serde(
        rename = "attributionResponseField",
        skip_serializing_if = "Option::is_none"
    )]
    pub attribution_response_field: Option<String>,
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "params")]
    pub params: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "responseType")]
    pub response_type: String,
    #[serde(rename = "resultsField")]
    pub results_field: String,
}

impl InfoPushDataContentSourceUnknown {
    pub fn new(
        info_push_data_content_source_base: models::InfoPushDataContentSourceBase,
        endpoint: String,
        kind: String,
        params: std::collections::HashMap<String, serde_json::Value>,
        response_type: String,
        results_field: String,
    ) -> InfoPushDataContentSourceUnknown {
        InfoPushDataContentSourceUnknown {
            info_push_data_content_source_base,
            attribution_response_field: None,
            endpoint,
            kind,
            params,
            response_type,
            results_field,
        }
    }
}
