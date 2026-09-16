use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserClientConfig {
    #[serde(rename = "accessReduceDecorAnim")]
    pub access_reduce_decor_anim: bool,
    #[serde(rename = "configString")]
    pub config_string: String,
}

impl UserClientConfig {
    pub fn new(access_reduce_decor_anim: bool, config_string: String) -> UserClientConfig {
        UserClientConfig {
            access_reduce_decor_anim,
            config_string,
        }
    }
}
