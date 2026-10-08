use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushIpsQuery {
    #[serde(rename = "include", skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
    #[serde(rename = "require", skip_serializing_if = "Option::is_none")]
    pub require: Option<String>,
}

impl InfoPushIpsQuery {
    pub fn new() -> InfoPushIpsQuery {
        InfoPushIpsQuery {
            include: None,
            require: None,
        }
    }
}
