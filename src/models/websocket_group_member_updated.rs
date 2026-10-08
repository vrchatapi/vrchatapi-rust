use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketGroupMemberUpdated {
    #[serde(rename = "member")]
    pub member: models::GroupMemberLimitedUser,
}

impl WebsocketGroupMemberUpdated {
    pub fn new(member: models::GroupMemberLimitedUser) -> WebsocketGroupMemberUpdated {
        WebsocketGroupMemberUpdated { member }
    }
}
