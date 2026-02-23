use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RegisterUserAccount200Response {
    CurrentUser(models::CurrentUser),
    RequiresTwoFactorAuth(models::RequiresTwoFactorAuth),
}

impl Default for RegisterUserAccount200Response {
    fn default() -> Self {
        Self::CurrentUser(Default::default())
    }
}
