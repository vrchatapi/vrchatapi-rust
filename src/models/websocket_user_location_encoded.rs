use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketUserLocationEncoded {
    #[serde_as(as = "serde_with::json::JsonString")]
    #[serde(rename = "content")]
    pub content: models::WebsocketUserLocation,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl WebsocketUserLocationEncoded {
    pub fn new(
        content: models::WebsocketUserLocation,
        r#type: Type,
    ) -> WebsocketUserLocationEncoded {
        WebsocketUserLocationEncoded { content, r#type }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "user-location")]
    UserLocation,
}

impl Default for Type {
    fn default() -> Type {
        Self::UserLocation
    }
}
