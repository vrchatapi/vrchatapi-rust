/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TwoFactorRequired {
    #[serde(rename = "requiresTwoFactorAuth")]
    pub requires_two_factor_auth: Vec<RequiresTwoFactorAuth>,
}

impl TwoFactorRequired {
    pub fn new(requires_two_factor_auth: Vec<RequiresTwoFactorAuth>) -> TwoFactorRequired {
        TwoFactorRequired {
            requires_two_factor_auth,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RequiresTwoFactorAuth {
    #[serde(rename = "totp")]
    Totp,
    #[serde(rename = "otp")]
    Otp,
    #[serde(rename = "emailOtp")]
    EmailOtp,
}

impl Default for RequiresTwoFactorAuth {
    fn default() -> RequiresTwoFactorAuth {
        Self::Totp
    }
}
