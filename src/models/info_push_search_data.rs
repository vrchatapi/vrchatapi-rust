use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushSearchData {
    #[serde(rename = "name")]
    pub name: models::LocalizableString,
    #[serde(rename = "search")]
    pub search: models::InfoPushDataSearch,
}

impl InfoPushSearchData {
    pub fn new(
        name: models::LocalizableString,
        search: models::InfoPushDataSearch,
    ) -> InfoPushSearchData {
        InfoPushSearchData { name, search }
    }
}
