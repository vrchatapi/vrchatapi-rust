use crate::models;
use serde::{Deserialize, Serialize};

/// LocalizedString : A string the client resolves through its localization table, falling back to `fallback` when the key is unknown.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalizedString {
    /// The text to show when `key` cannot be resolved.
    #[serde(rename = "fallback")]
    pub fallback: String,
    /// The localization key.
    #[serde(rename = "key")]
    pub key: String,
}

impl LocalizedString {
    /// A string the client resolves through its localization table, falling back to `fallback` when the key is unknown.
    pub fn new(fallback: String, key: String) -> LocalizedString {
        LocalizedString { fallback, key }
    }
}
