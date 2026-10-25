use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCodeApproved {
    #[serde(rename = "user")]
    pub user: models::CurrentUser,
}

impl DeviceCodeApproved {
    pub fn new(user: models::CurrentUser) -> DeviceCodeApproved {
        DeviceCodeApproved { user }
    }
}
