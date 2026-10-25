use crate::models;
use serde::{Deserialize, Serialize};

/// Where a device code was created, shown to the person about to approve it.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceAuthInfo {
    /// When the device code was created, in milliseconds since the Unix epoch.
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "geolocation")]
    pub geolocation: models::DeviceAuthInfoGeolocation,
}

impl DeviceAuthInfo {
    /// Where a device code was created, shown to the person about to approve it.
    pub fn new(created_at: i32, geolocation: models::DeviceAuthInfoGeolocation) -> DeviceAuthInfo {
        DeviceAuthInfo {
            created_at,
            geolocation,
        }
    }
}
