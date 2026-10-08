use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequiresTwoFactorAuth {
    #[serde(rename = "requiresTwoFactorAuth")]
    pub requires_two_factor_auth: Vec<models::TwoFactorAuthType>,
}

impl RequiresTwoFactorAuth {
    pub fn new(requires_two_factor_auth: Vec<models::TwoFactorAuthType>) -> RequiresTwoFactorAuth {
        RequiresTwoFactorAuth {
            requires_two_factor_auth,
        }
    }
}
