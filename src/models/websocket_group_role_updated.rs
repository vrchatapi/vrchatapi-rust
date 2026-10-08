use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketGroupRoleUpdated {
    #[serde(rename = "role")]
    pub role: models::GroupRole,
}

impl WebsocketGroupRoleUpdated {
    pub fn new(role: models::GroupRole) -> WebsocketGroupRoleUpdated {
        WebsocketGroupRoleUpdated { role }
    }
}
