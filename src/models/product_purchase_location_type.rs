use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProductPurchaseLocationType {
    #[serde(rename = "client_avatar_marketplace")]
    ClientAvatarMarketplace,
    #[serde(rename = "client_creator_store")]
    ClientCreatorStore,
    #[serde(rename = "client_group_store")]
    ClientGroupStore,
    #[serde(rename = "client_world_component")]
    ClientWorldComponent,
    #[serde(rename = "client_world_store")]
    ClientWorldStore,
    #[serde(rename = "undefined")]
    Undefined,
    #[serde(rename = "web_any")]
    WebAny,
    #[serde(rename = "web_avatar_marketplace")]
    WebAvatarMarketplace,
    #[serde(rename = "web_creator_store")]
    WebCreatorStore,
    #[serde(rename = "web_group_store")]
    WebGroupStore,
    #[serde(rename = "web_world_store")]
    WebWorldStore,
}

impl std::fmt::Display for ProductPurchaseLocationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ClientAvatarMarketplace => write!(f, "client_avatar_marketplace"),
            Self::ClientCreatorStore => write!(f, "client_creator_store"),
            Self::ClientGroupStore => write!(f, "client_group_store"),
            Self::ClientWorldComponent => write!(f, "client_world_component"),
            Self::ClientWorldStore => write!(f, "client_world_store"),
            Self::Undefined => write!(f, "undefined"),
            Self::WebAny => write!(f, "web_any"),
            Self::WebAvatarMarketplace => write!(f, "web_avatar_marketplace"),
            Self::WebCreatorStore => write!(f, "web_creator_store"),
            Self::WebGroupStore => write!(f, "web_group_store"),
            Self::WebWorldStore => write!(f, "web_world_store"),
        }
    }
}

impl Default for ProductPurchaseLocationType {
    fn default() -> ProductPurchaseLocationType {
        Self::ClientAvatarMarketplace
    }
}
