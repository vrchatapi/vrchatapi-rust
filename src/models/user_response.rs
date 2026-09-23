use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserResponse {
    User(models::User),
    CurrentUser(models::CurrentUser),
}

impl Default for UserResponse {
    fn default() -> Self {
        Self::User(Default::default())
    }
}
