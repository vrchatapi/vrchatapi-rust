/*
 * VRChat API Documentation
 *
 *
 * The version of the OpenAPI document: 1.3.0
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineObject {
    #[serde(rename = "code")]
    pub code: String,
}

impl InlineObject {
    pub fn new(code: String) -> InlineObject {
        InlineObject {
            code,
        }
    }
}


