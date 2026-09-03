use crate::models;
use serde::{Deserialize, Serialize};

/// SsoToken : A token for a third-party service.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SsoToken {
    #[serde(rename = "token")]
    pub token: String,
}

impl SsoToken {
    /// A token for a third-party service.
    pub fn new(token: String) -> SsoToken {
        SsoToken { token }
    }
}
