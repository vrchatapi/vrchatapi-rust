use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionAgreement {
    String(String),
    TransactionAgreement(Box<models::TransactionAgreement>),
}

impl Default for TransactionAgreement {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
