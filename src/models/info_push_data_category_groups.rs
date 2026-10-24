use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataCategoryGroups {
    #[serde(flatten)]
    pub info_push_data_category_base: models::InfoPushDataCategoryBase,
    #[serde(rename = "maxCells")]
    pub max_cells: i32,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl InfoPushDataCategoryGroups {
    pub fn new(
        info_push_data_category_base: models::InfoPushDataCategoryBase,
        max_cells: i32,
        r#type: Type,
    ) -> InfoPushDataCategoryGroups {
        InfoPushDataCategoryGroups {
            info_push_data_category_base,
            max_cells,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "groups")]
    Groups,
}

impl Default for Type {
    fn default() -> Type {
        Self::Groups
    }
}
