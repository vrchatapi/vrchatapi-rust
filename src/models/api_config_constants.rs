use crate::models;
use serde::{Deserialize, Serialize};

/// Constants
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiConfigConstants {
    /// Group-related constants
    #[serde(rename = "GROUPS")]
    pub groups: models::ApiConfigConstantsGroups,
    /// Instance-related constants
    #[serde(rename = "INSTANCE")]
    pub instance: models::ApiConfigConstantsInstance,
    /// Language-related constants
    #[serde(rename = "LANGUAGE")]
    pub language: models::ApiConfigConstantsLanguage,
}

impl ApiConfigConstants {
    /// Constants
    pub fn new(
        groups: models::ApiConfigConstantsGroups,
        instance: models::ApiConfigConstantsInstance,
        language: models::ApiConfigConstantsLanguage,
    ) -> ApiConfigConstants {
        ApiConfigConstants {
            groups,
            instance,
            language,
        }
    }
}
