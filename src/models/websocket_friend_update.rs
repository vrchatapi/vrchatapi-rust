use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketFriendUpdate {
    #[serde(rename = "user")]
    pub user: models::User,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl WebsocketFriendUpdate {
    pub fn new(user: models::User, user_id: String) -> WebsocketFriendUpdate {
        WebsocketFriendUpdate { user, user_id }
    }
}
