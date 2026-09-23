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
