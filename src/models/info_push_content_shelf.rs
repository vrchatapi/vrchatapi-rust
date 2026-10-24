use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushContentShelf {
    #[serde(flatten)]
    pub info_push_base: models::InfoPushBase,
    #[serde(rename = "data")]
    pub data: models::InfoPushContentShelfData,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl InfoPushContentShelf {
    pub fn new(
        info_push_base: models::InfoPushBase,
        data: models::InfoPushContentShelfData,
        r#type: Type,
    ) -> InfoPushContentShelf {
        InfoPushContentShelf {
            info_push_base,
            data,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ContentShelf")]
    ContentShelf,
}

impl Default for Type {
    fn default() -> Type {
        Self::ContentShelf
    }
}
