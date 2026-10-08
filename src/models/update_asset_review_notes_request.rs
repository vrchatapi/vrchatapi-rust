use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateAssetReviewNotesRequest {
    #[serde(rename = "reviewNotes")]
    pub review_notes: String,
}

impl UpdateAssetReviewNotesRequest {
    pub fn new(review_notes: String) -> UpdateAssetReviewNotesRequest {
        UpdateAssetReviewNotesRequest { review_notes }
    }
}
