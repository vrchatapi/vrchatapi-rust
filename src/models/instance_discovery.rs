use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstanceDiscovery {
    #[serde(rename = "attributionId")]
    pub attribution_id: String,
    #[serde(rename = "instances")]
    pub instances: Vec<models::LimitedInstance>,
}

impl InstanceDiscovery {
    pub fn new(
        attribution_id: String,
        instances: Vec<models::LimitedInstance>,
    ) -> InstanceDiscovery {
        InstanceDiscovery {
            attribution_id,
            instances,
        }
    }
}
