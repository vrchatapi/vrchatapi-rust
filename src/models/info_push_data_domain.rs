use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataDomain {
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl InfoPushDataDomain {
    pub fn new() -> InfoPushDataDomain {
        InfoPushDataDomain { domain: None }
    }
}
