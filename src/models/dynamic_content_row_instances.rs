use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicContentRowInstances {
    #[serde(flatten)]
    pub dynamic_content_row_base: models::DynamicContentRowBase,
    #[serde(rename = "groupPool", skip_serializing_if = "Option::is_none")]
    pub group_pool: Option<String>,
    #[serde(rename = "minOccupants")]
    pub min_occupants: i32,
    #[serde(rename = "mode")]
    pub mode: models::InstanceDiscoveryMode,
    #[serde(rename = "n")]
    pub n: i32,
    #[serde(rename = "params")]
    pub params: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "region", deserialize_with = "Option::deserialize")]
    pub region: Option<models::InstanceRegion>,
    #[serde(rename = "type", default)]
    pub r#type: Type,
    #[serde(rename = "worldPool", skip_serializing_if = "Option::is_none")]
    pub world_pool: Option<models::InstanceDiscoveryWorldPool>,
}

impl DynamicContentRowInstances {
    pub fn new(
        dynamic_content_row_base: models::DynamicContentRowBase,
        min_occupants: i32,
        mode: models::InstanceDiscoveryMode,
        n: i32,
        params: std::collections::HashMap<String, serde_json::Value>,
        region: Option<models::InstanceRegion>,
        r#type: Type,
    ) -> DynamicContentRowInstances {
        DynamicContentRowInstances {
            dynamic_content_row_base,
            group_pool: None,
            min_occupants,
            mode,
            n,
            params,
            region,
            r#type,
            world_pool: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "instances")]
    Instances,
}

impl Default for Type {
    fn default() -> Type {
        Self::Instances
    }
}
