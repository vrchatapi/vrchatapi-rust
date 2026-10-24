use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataContentSourceBase {
    #[serde(rename = "computedParams")]
    pub computed_params: std::collections::HashMap<String, String>,
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "pagination", deserialize_with = "Option::deserialize")]
    pub pagination: Option<models::InfoPushDataContentSourcePagination>,
    #[serde(rename = "paramModifiability")]
    pub param_modifiability: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "schemaVersion")]
    pub schema_version: i32,
}

impl InfoPushDataContentSourceBase {
    pub fn new(
        computed_params: std::collections::HashMap<String, String>,
        method: String,
        pagination: Option<models::InfoPushDataContentSourcePagination>,
        param_modifiability: std::collections::HashMap<String, serde_json::Value>,
        schema_version: i32,
    ) -> InfoPushDataContentSourceBase {
        InfoPushDataContentSourceBase {
            computed_params,
            method,
            pagination,
            param_modifiability,
            schema_version,
        }
    }
}
