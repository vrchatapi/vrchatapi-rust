use crate::models;
use serde::{Deserialize, Serialize};

/// ApiConfigEventShelfCampaign : A seasonal campaign a group event can be listed under.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiConfigEventShelfCampaign {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "name")]
    pub name: String,
}

impl ApiConfigEventShelfCampaign {
    /// A seasonal campaign a group event can be listed under.
    pub fn new(key: String, name: String) -> ApiConfigEventShelfCampaign {
        ApiConfigEventShelfCampaign {
            description: None,
            key,
            name,
        }
    }
}
