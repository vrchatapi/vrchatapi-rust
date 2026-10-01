use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUserNoteResponse {
    UserNote(models::UserNote),
    String(String),
}

impl Default for UpdateUserNoteResponse {
    fn default() -> Self {
        Self::UserNote(Default::default())
    }
}
