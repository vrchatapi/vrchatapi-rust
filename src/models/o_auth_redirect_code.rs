use crate::models;
use serde::{Deserialize, Serialize};

/// A short-lived code used to hand the current session to an OAuth redirect.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuthRedirectCode {
    #[serde(rename = "code")]
    pub code: String,
}

impl OAuthRedirectCode {
    /// A short-lived code used to hand the current session to an OAuth redirect.
    pub fn new(code: String) -> OAuthRedirectCode {
        OAuthRedirectCode { code }
    }
}
