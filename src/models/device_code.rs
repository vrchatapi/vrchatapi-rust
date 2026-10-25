use crate::models;
use serde::{Deserialize, Serialize};

/// A code that logs the device that created it in, once a logged-in user approves it with `approveDeviceAuth`.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCode {
    /// Identifies this login attempt to `checkDeviceCode`.
    #[serde(rename = "deviceCode")]
    pub device_code: String,
    #[serde(rename = "expiresIn")]
    pub expires_in: i32,
    #[serde(rename = "interval")]
    pub interval: i32,
    /// Identifies this login attempt to `getDeviceAuthInfo` and `approveDeviceAuth`.
    #[serde(rename = "userCode")]
    pub user_code: String,
    #[serde(rename = "verificationUrl")]
    pub verification_url: String,
    /// `verificationUrl` with the code already filled in.
    #[serde(rename = "verificationUrlComplete")]
    pub verification_url_complete: String,
}

impl DeviceCode {
    /// A code that logs the device that created it in, once a logged-in user approves it with `approveDeviceAuth`.
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
