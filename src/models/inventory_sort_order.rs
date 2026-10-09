use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InventorySortOrder {
    #[serde(rename = "alphabetic")]
    Alphabetic,
    #[serde(rename = "angry")]
    Angry,
    #[serde(rename = "happy")]
    Happy,
    #[serde(rename = "newest")]
    Newest,
    #[serde(rename = "newest_created")]
    NewestCreated,
    #[serde(rename = "newest_template_created")]
    NewestTemplateCreated,
    #[serde(rename = "newest_updated")]
    NewestUpdated,
    #[serde(rename = "oldest")]
    Oldest,
    #[serde(rename = "oldest_created")]
    OldestCreated,
    #[serde(rename = "oldest_template_created")]
    OldestTemplateCreated,
    #[serde(rename = "oldest_updated")]
    OldestUpdated,
    #[serde(rename = "reverse-orthographic")]
    ReverseOrthographic,
}

impl std::fmt::Display for InventorySortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Alphabetic => write!(f, "alphabetic"),
            Self::Angry => write!(f, "angry"),
            Self::Happy => write!(f, "happy"),
            Self::Newest => write!(f, "newest"),
            Self::NewestCreated => write!(f, "newest_created"),
            Self::NewestTemplateCreated => write!(f, "newest_template_created"),
            Self::NewestUpdated => write!(f, "newest_updated"),
            Self::Oldest => write!(f, "oldest"),
            Self::OldestCreated => write!(f, "oldest_created"),
            Self::OldestTemplateCreated => write!(f, "oldest_template_created"),
            Self::OldestUpdated => write!(f, "oldest_updated"),
            Self::ReverseOrthographic => write!(f, "reverse-orthographic"),
        }
    }
}

impl Default for InventorySortOrder {
    fn default() -> InventorySortOrder {
        Self::Alphabetic
    }
}
