use crate::models;
use serde::{Deserialize, Serialize};

/// GroupGalleryImageList : A page of a group gallery's images.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupGalleryImageList {
    #[serde(rename = "results")]
    pub results: Vec<models::GroupGalleryImage>,
    #[serde(rename = "totalCount")]
    pub total_count: i32,
}

impl GroupGalleryImageList {
    /// A page of a group gallery's images.
    pub fn new(results: Vec<models::GroupGalleryImage>, total_count: i32) -> GroupGalleryImageList {
        GroupGalleryImageList {
            results,
            total_count,
        }
    }
}
