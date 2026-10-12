use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CurrentUserLoginResponse {
    CurrentUser(models::CurrentUser),
    RequiresTwoFactorAuth(models::RequiresTwoFactorAuth),
}

impl Default for CurrentUserLoginResponse {
    fn default() -> Self {
        Self::CurrentUser(Default::default())
    }
}

impl From<models::CurrentUser> for CurrentUserLoginResponse {
    fn from(value: models::CurrentUser) -> Self {
        Self::CurrentUser(value)
    }
}

impl TryFrom<CurrentUserLoginResponse> for models::CurrentUser {
    type Error = CurrentUserLoginResponse;

    fn try_from(value: CurrentUserLoginResponse) -> Result<Self, Self::Error> {
        match value {
            CurrentUserLoginResponse::CurrentUser(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::RequiresTwoFactorAuth> for CurrentUserLoginResponse {
    fn from(value: models::RequiresTwoFactorAuth) -> Self {
        Self::RequiresTwoFactorAuth(value)
    }
}

impl TryFrom<CurrentUserLoginResponse> for models::RequiresTwoFactorAuth {
    type Error = CurrentUserLoginResponse;

    fn try_from(value: CurrentUserLoginResponse) -> Result<Self, Self::Error> {
        match value {
            CurrentUserLoginResponse::RequiresTwoFactorAuth(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
