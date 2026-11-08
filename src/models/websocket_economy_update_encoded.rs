use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketEconomyUpdateEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketEconomyUpdate,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketEconomyUpdateEncoded {
    pub fn new(
        content: models::WebsocketEconomyUpdate,
        r#type: Type,
    ) -> WebsocketEconomyUpdateEncoded {
        WebsocketEconomyUpdateEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "economy-update")]
    EconomyUpdate,
}

impl Default for Type {
    fn default() -> Type {
        Self::EconomyUpdate
    }
}
