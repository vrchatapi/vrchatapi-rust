use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PastDisplayName {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "reverted")]
    pub reverted: bool,
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl PastDisplayName {
    pub fn new(
        display_name: String,
        reverted: bool,
        updated_at: chrono::DateTime<chrono::FixedOffset>,
    ) -> PastDisplayName {
        PastDisplayName {
            display_name,
            reverted,
            updated_at,
        }
    }
}
