use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserResponse {
    CurrentUser(models::CurrentUser),
    User(models::User),
}

impl Default for UserResponse {
    fn default() -> Self {
        Self::CurrentUser(Default::default())
    }
}

impl From<models::CurrentUser> for UserResponse {
    fn from(value: models::CurrentUser) -> Self {
        Self::CurrentUser(value)
    }
}

impl TryFrom<UserResponse> for models::CurrentUser {
    type Error = UserResponse;

    fn try_from(value: UserResponse) -> Result<Self, Self::Error> {
        match value {
            UserResponse::CurrentUser(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::User> for UserResponse {
    fn from(value: models::User) -> Self {
        Self::User(value)
    }
}

impl TryFrom<UserResponse> for models::User {
    type Error = UserResponse;

    fn try_from(value: UserResponse) -> Result<Self, Self::Error> {
        match value {
            UserResponse::User(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
