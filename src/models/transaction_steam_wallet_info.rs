/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionSteamWalletInfo {
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "status")]
    pub status: String,
}

impl TransactionSteamWalletInfo {
    pub fn new(state: String, country: String, currency: String, status: String) -> TransactionSteamWalletInfo {
        TransactionSteamWalletInfo {
            state,
            country,
            currency,
            status,
        }
    }
}


