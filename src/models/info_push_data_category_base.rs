use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataCategoryBase {
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
    #[serde(rename = "name")]
    pub name: models::LocalizableString,
}

impl InfoPushDataCategoryBase {
    pub fn new(ids: Vec<String>, name: models::LocalizableString) -> InfoPushDataCategoryBase {
        InfoPushDataCategoryBase { ids, name }
    }
}
