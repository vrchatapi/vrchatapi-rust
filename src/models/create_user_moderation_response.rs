use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateUserModerationResponse {
    #[serde(rename = "id")]
    pub id: String,
}

impl CreateUserModerationResponse {
    pub fn new(id: String) -> CreateUserModerationResponse {
        CreateUserModerationResponse { id }
    }
}
