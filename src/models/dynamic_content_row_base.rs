use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicContentRowBase {
    #[serde(rename = "name")]
    pub name: models::LocalizableString,
    /// Usually \"ThisPlatformSupported\", but can also be other values such as \"all\" or platform specific identifiers.
    #[serde(rename = "platform")]
    pub platform: String,
}

impl DynamicContentRowBase {
    pub fn new(name: models::LocalizableString, platform: String) -> DynamicContentRowBase {
        DynamicContentRowBase { name, platform }
    }
}
