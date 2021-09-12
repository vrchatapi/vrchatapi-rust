/*
 * VRChat API Documentation
 *
 *
 * The version of the OpenAPI document: 1.3.0
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2002 {
    #[serde(rename = "ok")]
    pub ok: bool,
    #[serde(rename = "serverName")]
    pub server_name: String,
    #[serde(rename = "buildVersionTag")]
    pub build_version_tag: String,
}

impl InlineResponse2002 {
    pub fn new(ok: bool, server_name: String, build_version_tag: String) -> InlineResponse2002 {
        InlineResponse2002 {
            ok,
            server_name,
            build_version_tag,
        }
    }
}


