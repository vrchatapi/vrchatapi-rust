use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EconomyPayout {
    #[serde(rename = "paymentAmountTokens")]
    pub payment_amount_tokens: i32,
    #[serde(rename = "paymentAmountUsd")]
    pub payment_amount_usd: i32,
    #[serde(rename = "paymentCreated")]
    pub payment_created: String,
    #[serde(rename = "paymentOutId")]
    pub payment_out_id: i32,
    #[serde(rename = "paymentPlatform")]
    pub payment_platform: String,
    #[serde(rename = "paymentPlatformCode")]
    pub payment_platform_code: i32,
    #[serde(rename = "paymentStatus")]
    pub payment_status: String,
    #[serde(rename = "paymentStatusCode")]
    pub payment_status_code: i32,
    #[serde(rename = "paymentUpdated")]
    pub payment_updated: String,
    #[serde(
        rename = "platformPaymentGuid",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub platform_payment_guid: Option<Option<String>>,
    #[serde(
        rename = "platformPaymentMethod",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub platform_payment_method: Option<Option<String>>,
    #[serde(
        rename = "reversalDate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub reversal_date: Option<Option<String>>,
    #[serde(
        rename = "reversalReason",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub reversal_reason: Option<Option<String>>,
    #[serde(
        rename = "reversalReasonCode",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub reversal_reason_code: Option<Option<i32>>,
    #[serde(
        rename = "reversalTransactionId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub reversal_transaction_id: Option<Option<i32>>,
    #[serde(rename = "transactionId")]
    pub transaction_id: i32,
}

impl EconomyPayout {
    pub fn new(
        payment_amount_tokens: i32,
        payment_amount_usd: i32,
        payment_created: String,
        payment_out_id: i32,
        payment_platform: String,
        payment_platform_code: i32,
        payment_status: String,
        payment_status_code: i32,
        payment_updated: String,
        transaction_id: i32,
    ) -> EconomyPayout {
        EconomyPayout {
            payment_amount_tokens,
            payment_amount_usd,
            payment_created,
            payment_out_id,
            payment_platform,
            payment_platform_code,
            payment_status,
            payment_status_code,
            payment_updated,
            platform_payment_guid: None,
            platform_payment_method: None,
            reversal_date: None,
            reversal_reason: None,
            reversal_reason_code: None,
            reversal_transaction_id: None,
            transaction_id,
        }
    }
}
