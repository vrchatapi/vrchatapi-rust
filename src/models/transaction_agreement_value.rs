use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionAgreementValue {
    String(String),
    TransactionAgreement(models::TransactionAgreement),
}

impl Default for TransactionAgreementValue {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

impl From<models::TransactionAgreement> for TransactionAgreementValue {
    fn from(value: models::TransactionAgreement) -> Self {
        Self::TransactionAgreement(value)
    }
}

impl TryFrom<TransactionAgreementValue> for models::TransactionAgreement {
    type Error = TransactionAgreementValue;

    fn try_from(value: TransactionAgreementValue) -> Result<Self, Self::Error> {
        match value {
            TransactionAgreementValue::TransactionAgreement(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
