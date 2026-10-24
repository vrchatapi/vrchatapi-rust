use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchAvatarsResult {
    ArrayVecAvatar(Vec<models::Avatar>),
    AvatarList(models::AvatarList),
}

impl Default for SearchAvatarsResult {
    fn default() -> Self {
        Self::ArrayVecAvatar(Default::default())
    }
}

impl From<models::AvatarList> for SearchAvatarsResult {
    fn from(value: models::AvatarList) -> Self {
        Self::AvatarList(value)
    }
}

impl TryFrom<SearchAvatarsResult> for models::AvatarList {
    type Error = SearchAvatarsResult;

    fn try_from(value: SearchAvatarsResult) -> Result<Self, Self::Error> {
        match value {
            SearchAvatarsResult::AvatarList(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
