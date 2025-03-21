/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AgeVerificationStatus : `verified` is obsolete.  User who have verified and are 18+ can switch to `plus18` status.
/// `verified` is obsolete.  User who have verified and are 18+ can switch to `plus18` status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AgeVerificationStatus {
    #[serde(rename = "hidden")]
    hidden,
    #[serde(rename = "verified")]
    verified,
    #[serde(rename = "18+")]
    plus18,
}

impl std::fmt::Display for AgeVerificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::hidden => write!(f, "hidden"),
            Self::verified => write!(f, "verified"),
            Self::plus18 => write!(f, "18+"),
        }
    }
}

impl Default for AgeVerificationStatus {
    fn default() -> AgeVerificationStatus {
        Self::hidden
    }
}
