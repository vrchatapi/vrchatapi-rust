use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketUserBadgeUnassigned {
    #[serde(rename = "badgeId")]
    pub badge_id: String,
}

impl WebsocketUserBadgeUnassigned {
    pub fn new(badge_id: String) -> WebsocketUserBadgeUnassigned {
        WebsocketUserBadgeUnassigned { badge_id }
    }
}
