use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LocalizableString {
    String(String),
    LocalizedString(models::LocalizedString),
}

impl Default for LocalizableString {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

impl From<models::LocalizedString> for LocalizableString {
    fn from(value: models::LocalizedString) -> Self {
        Self::LocalizedString(value)
    }
}

impl TryFrom<LocalizableString> for models::LocalizedString {
    type Error = LocalizableString;

    fn try_from(value: LocalizableString) -> Result<Self, Self::Error> {
        match value {
            LocalizableString::LocalizedString(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
