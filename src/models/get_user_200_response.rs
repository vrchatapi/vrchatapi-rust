use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUser200Response {
    User(models::User),
    CurrentUser(models::CurrentUser),
}

impl Default for GetUser200Response {
    fn default() -> Self {
        Self::User(Default::default())
    }
}
