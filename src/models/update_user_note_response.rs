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

impl From<models::UserNote> for UpdateUserNoteResponse {
    fn from(value: models::UserNote) -> Self {
        Self::UserNote(value)
    }
}

impl TryFrom<UpdateUserNoteResponse> for models::UserNote {
    type Error = UpdateUserNoteResponse;

    fn try_from(value: UpdateUserNoteResponse) -> Result<Self, Self::Error> {
        match value {
            UpdateUserNoteResponse::UserNote(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
