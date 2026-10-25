use crate::models;
use serde::{Deserialize, Serialize};

/// Where and when a device code was created.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceAuthInfo {
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "geolocation")]
    pub geolocation: models::DeviceAuthInfoGeolocation,
}

impl DeviceAuthInfo {
    /// Where and when a device code was created.
    pub fn new(created_at: i32, geolocation: models::DeviceAuthInfoGeolocation) -> DeviceAuthInfo {
        DeviceAuthInfo {
            created_at,
            geolocation,
        }
    }
}
