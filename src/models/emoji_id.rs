use crate::models;
use serde::{Deserialize, Serialize};

/// A default emoji, or the FileID of an uploaded one.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmojiId {
    DefaultEmojiID(String),
    FileID(String),
}

impl Default for EmojiId {
    fn default() -> Self {
        Self::DefaultEmojiID(Default::default())
    }
}
