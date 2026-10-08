use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SellerEligibility {
    #[serde(rename = "eligible")]
    pub eligible: bool,
}

impl SellerEligibility {
    pub fn new(eligible: bool) -> SellerEligibility {
        SellerEligibility { eligible }
    }
}
