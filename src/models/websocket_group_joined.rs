use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketGroupJoined {
    #[serde(rename = "groupId")]
    pub group_id: String,
}

impl WebsocketGroupJoined {
    pub fn new(group_id: String) -> WebsocketGroupJoined {
        WebsocketGroupJoined { group_id }
    }
}
