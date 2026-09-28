use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataContentSource {
    #[serde(
        rename = "attributionResponseField",
        skip_serializing_if = "Option::is_none"
    )]
    pub attribution_response_field: Option<String>,
    #[serde(rename = "computedParams")]
    pub computed_params: std::collections::HashMap<String, String>,
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "pagination", deserialize_with = "Option::deserialize")]
    pub pagination: Option<models::InfoPushDataContentSourcePagination>,
    #[serde(rename = "paramModifiability")]
    pub param_modifiability: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "params")]
    pub params: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "responseType")]
    pub response_type: String,
    #[serde(rename = "resultsField")]
    pub results_field: String,
    #[serde(rename = "schemaVersion")]
    pub schema_version: i32,
}

impl InfoPushDataContentSource {
    pub fn new(
        computed_params: std::collections::HashMap<String, String>,
        endpoint: String,
        kind: String,
        method: String,
        pagination: Option<models::InfoPushDataContentSourcePagination>,
        param_modifiability: std::collections::HashMap<String, serde_json::Value>,
        params: std::collections::HashMap<String, serde_json::Value>,
        response_type: String,
        results_field: String,
        schema_version: i32,
    ) -> InfoPushDataContentSource {
        InfoPushDataContentSource {
            attribution_response_field: None,
            computed_params,
            endpoint,
            kind,
            method,
            pagination,
            param_modifiability,
            params,
            response_type,
            results_field,
            schema_version,
        }
    }
}
