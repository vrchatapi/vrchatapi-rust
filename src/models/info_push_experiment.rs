use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushExperiment {
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "variant", skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

impl InfoPushExperiment {
    pub fn new() -> InfoPushExperiment {
        InfoPushExperiment {
            key: None,
            variant: None,
        }
    }
}
