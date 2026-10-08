use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionAgreementOneOf {
    String(String),
    TransactionAgreement(models::TransactionAgreement),
}

impl Default for TransactionAgreementOneOf {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
