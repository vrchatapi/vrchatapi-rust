use crate::models;
use serde::{Deserialize, Serialize};

/// UpdateUserClientConfigRequest : Only the settings named are changed.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateUserClientConfigRequest {
    #[serde(
        rename = "accessReduceDecorAnim",
        skip_serializing_if = "Option::is_none"
    )]
    pub access_reduce_decor_anim: Option<bool>,
}

impl UpdateUserClientConfigRequest {
    /// Only the settings named are changed.
    pub fn new() -> UpdateUserClientConfigRequest {
        UpdateUserClientConfigRequest {
            access_reduce_decor_anim: None,
        }
    }
}
