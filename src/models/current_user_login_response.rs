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
