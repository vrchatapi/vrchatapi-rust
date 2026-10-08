use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataPresentation {
    #[serde(rename = "layout")]
    pub layout: String,
}

impl InfoPushDataPresentation {
    pub fn new(layout: String) -> InfoPushDataPresentation {
        InfoPushDataPresentation { layout }
    }
}
