use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketEconomyUpdate {
    #[serde(rename = "dirtyPurchases")]
    pub dirty_purchases: bool,
}

impl WebsocketEconomyUpdate {
    pub fn new(dirty_purchases: bool) -> WebsocketEconomyUpdate {
        WebsocketEconomyUpdate { dirty_purchases }
    }
}
