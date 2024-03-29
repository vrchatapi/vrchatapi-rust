/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<crate::models::Response>>,
}

impl Error {
    pub fn new() -> Error {
        Error {
            error: None,
        }
    }
}


