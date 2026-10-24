use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstanceDiscoveryBoost {
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "friends")]
    Friends,
    #[serde(rename = "myEvents")]
    MyEvents,
    #[serde(rename = "myFavoriteWorlds")]
    MyFavoriteWorlds,
    #[serde(rename = "myGroupEvents")]
    MyGroupEvents,
    #[serde(rename = "myGroups")]
    MyGroups,
}

impl std::fmt::Display for InstanceDiscoveryBoost {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "ALL"),
            Self::Friends => write!(f, "friends"),
            Self::MyEvents => write!(f, "myEvents"),
            Self::MyFavoriteWorlds => write!(f, "myFavoriteWorlds"),
            Self::MyGroupEvents => write!(f, "myGroupEvents"),
            Self::MyGroups => write!(f, "myGroups"),
        }
    }
}

impl Default for InstanceDiscoveryBoost {
    fn default() -> InstanceDiscoveryBoost {
        Self::All
    }
}
