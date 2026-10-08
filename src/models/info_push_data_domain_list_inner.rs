use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataDomainListInner {
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl InfoPushDataDomainListInner {
    pub fn new() -> InfoPushDataDomainListInner {
        InfoPushDataDomainListInner { domain: None }
    }
}
