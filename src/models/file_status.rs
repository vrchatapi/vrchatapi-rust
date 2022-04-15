/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FileStatus {
    #[serde(rename = "waiting")]
    Waiting,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "queued")]
    Queued,

}

impl ToString for FileStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Waiting => String::from("waiting"),
            Self::Complete => String::from("complete"),
            Self::None => String::from("none"),
            Self::Queued => String::from("queued"),
        }
    }
}

impl Default for FileStatus {
    fn default() -> FileStatus {
        Self::Waiting
    }
}




