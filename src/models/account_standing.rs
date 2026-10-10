use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountStanding {
    #[serde(rename = "issueClearDays")]
    pub issue_clear_days: i32,
    #[serde(rename = "sanctions")]
    pub sanctions: Vec<serde_json::Value>,
    #[serde(rename = "standing")]
    pub standing: String,
}

impl AccountStanding {
    pub fn new(
        issue_clear_days: i32,
        sanctions: Vec<serde_json::Value>,
        standing: String,
    ) -> AccountStanding {
        AccountStanding {
            issue_clear_days,
            sanctions,
            standing,
        }
    }
}
