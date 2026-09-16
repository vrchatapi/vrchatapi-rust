use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGroupGalleryImages200Response {
    ArrayVecGroupGalleryImage(Vec<models::GroupGalleryImage>),
    GroupGalleryImageList(models::GroupGalleryImageList),
}

impl Default for GetGroupGalleryImages200Response {
    fn default() -> Self {
        Self::ArrayVecGroupGalleryImage(Default::default())
    }
}
