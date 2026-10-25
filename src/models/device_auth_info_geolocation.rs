use crate::models;
use serde::{Deserialize, Serialize};

/// The approximate location of the device that created the code.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceAuthInfoGeolocation {
    #[serde(rename = "city")]
    pub city: String,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "regionCode")]
    pub region_code: String,
}

impl DeviceAuthInfoGeolocation {
    /// The approximate location of the device that created the code.
    pub fn new(
        city: String,
        country: String,
        region: String,
        region_code: String,
    ) -> DeviceAuthInfoGeolocation {
        DeviceAuthInfoGeolocation {
            city,
            country,
            region,
            region_code,
        }
    }
}
