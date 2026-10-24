use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataCategoryUnknown {
    #[serde(flatten)]
    pub info_push_data_category_base: models::InfoPushDataCategoryBase,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl InfoPushDataCategoryUnknown {
    pub fn new(
        info_push_data_category_base: models::InfoPushDataCategoryBase,
        r#type: String,
    ) -> InfoPushDataCategoryUnknown {
        InfoPushDataCategoryUnknown {
            info_push_data_category_base,
            r#type,
        }
    }
}
