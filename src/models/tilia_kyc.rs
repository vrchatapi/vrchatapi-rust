use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiliaKyc {
    /// Tilia account identifier.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// KYC verification identifier.
    #[serde(rename = "kyc_id")]
    pub kyc_id: String,
    /// Requirement state reported by Tilia.
    #[serde(rename = "kyc_requirements")]
    pub kyc_requirements: String,
    /// Match checks returned by Tilia.
    #[serde(rename = "match_checks")]
    pub match_checks: Vec<String>,
    /// PII verification level.
    #[serde(rename = "pii_level")]
    pub pii_level: String,
    /// Additional rules returned by Tilia.
    #[serde(rename = "rules")]
    pub rules: Vec<String>,
    /// Overall KYC state.
    #[serde(rename = "state")]
    pub state: String,
    /// Retry rule code returned by Tilia, if any.
    #[serde(rename = "tilia_retry_rule_code")]
    pub tilia_retry_rule_code: String,
}

impl TiliaKyc {
    pub fn new(
        account_id: String,
        kyc_id: String,
        kyc_requirements: String,
        match_checks: Vec<String>,
        pii_level: String,
        rules: Vec<String>,
        state: String,
        tilia_retry_rule_code: String,
    ) -> TiliaKyc {
        TiliaKyc {
            account_id,
            kyc_id,
            kyc_requirements,
            match_checks,
            pii_level,
            rules,
            state,
            tilia_retry_rule_code,
        }
    }
}
