use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushUrlConsentScreenData {
    #[serde(rename = "disclaimerText")]
    pub disclaimer_text: String,
    #[serde(rename = "domainList")]
    pub domain_list: Vec<models::InfoPushDataDomain>,
    #[serde(rename = "worldTag")]
    pub world_tag: String,
}

impl InfoPushUrlConsentScreenData {
    pub fn new(
        disclaimer_text: String,
        domain_list: Vec<models::InfoPushDataDomain>,
        world_tag: String,
    ) -> InfoPushUrlConsentScreenData {
        InfoPushUrlConsentScreenData {
            disclaimer_text,
            domain_list,
            world_tag,
        }
    }
}
