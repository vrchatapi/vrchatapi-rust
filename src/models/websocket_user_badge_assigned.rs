use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketUserBadgeAssigned {
    #[serde(rename = "badge")]
    pub badge: models::Badge,
}

impl WebsocketUserBadgeAssigned {
    pub fn new(badge: models::Badge) -> WebsocketUserBadgeAssigned {
        WebsocketUserBadgeAssigned { badge }
    }
}
