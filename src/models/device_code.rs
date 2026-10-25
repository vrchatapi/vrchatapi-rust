use crate::models;
use serde::{Deserialize, Serialize};

/// A code that logs this device in once someone approves it at `verificationUrl`.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCode {
    /// Identifies this login attempt to `checkDeviceCode`.
    #[serde(rename = "deviceCode")]
    pub device_code: String,
    /// Seconds until the code expires.
    #[serde(rename = "expiresIn")]
    pub expires_in: i32,
    /// Seconds to wait between calls to `checkDeviceCode`.
    #[serde(rename = "interval")]
    pub interval: i32,
    /// The code a person enters at `verificationUrl` to approve the login.
    #[serde(rename = "userCode")]
    pub user_code: String,
    #[serde(rename = "verificationUrl")]
    pub verification_url: String,
    /// `verificationUrl` with the code already filled in.
    #[serde(rename = "verificationUrlComplete")]
    pub verification_url_complete: String,
}

impl DeviceCode {
    /// A code that logs this device in once someone approves it at `verificationUrl`.
    pub fn new(
        device_code: String,
        expires_in: i32,
        interval: i32,
        user_code: String,
        verification_url: String,
        verification_url_complete: String,
    ) -> DeviceCode {
        DeviceCode {
            device_code,
            expires_in,
            interval,
            user_code,
            verification_url,
            verification_url_complete,
        }
    }
}
