use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupGalleryImageListResponse {
    ArrayVecGroupGalleryImage(Vec<models::GroupGalleryImage>),
    GroupGalleryImageList(models::GroupGalleryImageList),
}

impl Default for GroupGalleryImageListResponse {
    fn default() -> Self {
        Self::ArrayVecGroupGalleryImage(Default::default())
    }
}

impl From<models::GroupGalleryImageList> for GroupGalleryImageListResponse {
    fn from(value: models::GroupGalleryImageList) -> Self {
        Self::GroupGalleryImageList(value)
    }
}

impl TryFrom<GroupGalleryImageListResponse> for models::GroupGalleryImageList {
    type Error = GroupGalleryImageListResponse;

    fn try_from(value: GroupGalleryImageListResponse) -> Result<Self, Self::Error> {
        match value {
            GroupGalleryImageListResponse::GroupGalleryImageList(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
