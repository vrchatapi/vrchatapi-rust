use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvatarUnityPackageUrlObject {
    #[serde(rename = "unityPackageUrl", skip_serializing_if = "Option::is_none")]
    pub unity_package_url: Option<String>,
}

impl AvatarUnityPackageUrlObject {
    pub fn new() -> AvatarUnityPackageUrlObject {
        AvatarUnityPackageUrlObject {
            unity_package_url: None,
        }
    }
}
