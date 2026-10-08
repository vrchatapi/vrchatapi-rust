use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WebsocketContentRefreshActionType {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "deleted")]
    Deleted,
}

impl std::fmt::Display for WebsocketContentRefreshActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "add"),
            Self::Created => write!(f, "created"),
            Self::Delete => write!(f, "delete"),
            Self::Deleted => write!(f, "deleted"),
        }
    }
}

impl Default for WebsocketContentRefreshActionType {
    fn default() -> WebsocketContentRefreshActionType {
        Self::Add
    }
}
