use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataControlOption {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "label")]
    pub label: models::LocalizedString,
    #[serde(rename = "params")]
    pub params: std::collections::HashMap<String, serde_json::Value>,
}

impl InfoPushDataControlOption {
    pub fn new(
        id: String,
        label: models::LocalizedString,
        params: std::collections::HashMap<String, serde_json::Value>,
    ) -> InfoPushDataControlOption {
        InfoPushDataControlOption { id, label, params }
    }
}
