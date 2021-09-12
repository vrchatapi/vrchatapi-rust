/*
 * VRChat API Documentation
 *
 *
 * The version of the OpenAPI document: 1.3.0
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReleaseStatus {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "hidden")]
    Hidden,

}

impl ToString for ReleaseStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Public => String::from("public"),
            Self::Private => String::from("private"),
            Self::Hidden => String::from("hidden"),
        }
    }
}




