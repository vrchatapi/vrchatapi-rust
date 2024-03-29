/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
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
    #[serde(rename = "all")]
    All,

}

impl ToString for ReleaseStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Public => String::from("public"),
            Self::Private => String::from("private"),
            Self::Hidden => String::from("hidden"),
            Self::All => String::from("all"),
        }
    }
}

impl Default for ReleaseStatus {
    fn default() -> ReleaseStatus {
        Self::Public
    }
}




