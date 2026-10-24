use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushUntyped {
    #[serde(flatten)]
    pub info_push_base: models::InfoPushBase,
    #[serde(rename = "data")]
    pub data: models::InfoPushData,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl InfoPushUntyped {
    pub fn new(
        info_push_base: models::InfoPushBase,
        data: models::InfoPushData,
    ) -> InfoPushUntyped {
        InfoPushUntyped {
            info_push_base,
            data,
            r#type: None,
        }
    }
}
